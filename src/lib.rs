//! Runs orthra's language server for Zed.
//!
//! The binary is the first of:
//! 1. what `lsp.orthra.binary.path` names;
//! 2. `orthra` on the worktree's `PATH`, where `cargo install` puts it;
//! 3. the latest release of orthra for this platform, downloaded once into the extension's own
//!    directory. A newer release replaces it, and without a network the one downloaded before
//!    still serves.

use std::fs;

use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

/// The server's name in Zed's settings: `lsp.orthra`.
const SERVER: &str = "orthra";

/// Where the releases are. Each has `orthra-<target>.tar.gz`, holding `orthra-<target>/orthra`
/// (orthra's `tools/dist/pack.sh`).
const REPOSITORY: &str = "koroblabs/orthra";

/// The prefix of every directory a release is downloaded into: `orthra-v0.1.0`.
const DOWNLOADED: &str = "orthra-";

struct Orthra {
    /// The downloaded binary, once found in this session.
    downloaded: Option<String>,
}

impl Orthra {
    /// The downloaded binary: the latest release's, fetched if it is not here yet.
    fn downloaded(&mut self, id: &LanguageServerId) -> Result<String> {
        if let Some(path) = &self.downloaded {
            if is_file(path) {
                return Ok(path.clone());
            }
        }
        let target = target()?;
        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = match zed::latest_github_release(
            REPOSITORY,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(release) => release,
            Err(err) => {
                // Offline, or GitHub said no: the release downloaded before is still orthra.
                if let Some(path) = any_downloaded(target) {
                    zed::set_language_server_installation_status(
                        id,
                        &zed::LanguageServerInstallationStatus::None,
                    );
                    self.downloaded = Some(path.clone());
                    return Ok(path);
                }
                return Err(failed(
                    id,
                    format!("could not ask GitHub for orthra's latest release: {err}"),
                ));
            }
        };

        let archive = format!("orthra-{target}.tar.gz");
        let Some(asset) = release.assets.iter().find(|asset| asset.name == archive) else {
            return Err(failed(
                id,
                format!("orthra {} has no {archive}", release.version),
            ));
        };
        let dir = format!("{DOWNLOADED}{}", release.version);
        let binary = format!("{dir}/orthra-{target}/orthra");
        if !is_file(&binary) {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(&asset.download_url, &dir, zed::DownloadedFileType::GzipTar)
                .map_err(|err| {
                    failed(id, format!("downloading orthra {}: {err}", release.version))
                })?;
            zed::make_file_executable(&binary).map_err(|err| failed(id, err))?;
            remove_all_but(&dir);
        }
        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::None,
        );
        self.downloaded = Some(binary.clone());
        Ok(binary)
    }
}

impl zed::Extension for Orthra {
    fn new() -> Self {
        Orthra { downloaded: None }
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let settings = LspSettings::for_worktree(SERVER, worktree)?;
        if let Some(binary) = settings.binary {
            if let Some(command) = binary.path {
                return Ok(zed::Command {
                    command,
                    args: binary.arguments.unwrap_or_else(|| vec!["serve".to_owned()]),
                    env: binary.env.unwrap_or_default().into_iter().collect(),
                });
            }
        }
        let command = match worktree.which("orthra") {
            Some(path) => path,
            None => self.downloaded(id)?,
        };
        Ok(zed::Command {
            command,
            args: vec!["serve".to_owned()],
            env: Default::default(),
        })
    }

    /// `lsp.orthra.initialization_options`, as they are: `{"languages": ["en-US", "uk-UA"]}`.
    fn language_server_initialization_options(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree(SERVER, worktree)?.initialization_options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree(SERVER, worktree)?.settings)
    }
}

/// The release's name for this platform.
fn target() -> Result<&'static str> {
    match zed::current_platform() {
        (zed::Os::Mac, zed::Architecture::Aarch64) => Ok("aarch64-apple-darwin"),
        (zed::Os::Mac, zed::Architecture::X8664) => Ok("x86_64-apple-darwin"),
        (zed::Os::Linux, zed::Architecture::Aarch64) => Ok("aarch64-unknown-linux-gnu"),
        (zed::Os::Linux, zed::Architecture::X8664) => Ok("x86_64-unknown-linux-gnu"),
        _ => Err("orthra has no release for this platform: build it with \
                  `cargo install --locked --git https://github.com/koroblabs/orthra orthra-cli` \
                  and put it on PATH, or set `lsp.orthra.binary.path`"
            .to_owned()),
    }
}

/// A binary downloaded before, for this platform, if there is one.
fn any_downloaded(target: &str) -> Option<String> {
    fs::read_dir(".")
        .ok()?
        .flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.starts_with(DOWNLOADED))
        .map(|name| format!("{name}/orthra-{target}/orthra"))
        .find(|binary| is_file(binary))
}

/// Removes every downloaded release but `keep`.
fn remove_all_but(keep: &str) {
    let Ok(entries) = fs::read_dir(".") else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if name.starts_with(DOWNLOADED) && name != keep {
            // A directory that cannot be removed is only disk space.
            let _ = fs::remove_dir_all(entry.path());
        }
    }
}

/// Says in Zed that installing failed, and returns why.
fn failed(id: &LanguageServerId, why: String) -> String {
    zed::set_language_server_installation_status(
        id,
        &zed::LanguageServerInstallationStatus::Failed(why.clone()),
    );
    why
}

fn is_file(path: &str) -> bool {
    fs::metadata(path).is_ok_and(|metadata| metadata.is_file())
}

zed::register_extension!(Orthra);
