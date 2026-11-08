# orthra for Zed

[orthra](https://github.com/koroblabs/orthra) is a spell-checking language server that is fast and
quiet. It checks prose, and the comments, strings and declared names in code. A word your project
declares, in its manifest or in its code, is a word, and it runs entirely on your machine.

- **Prose:** plain text, Markdown, reStructuredText and commit messages.
- **Code:** Rust, TypeScript, JavaScript, Python, Go, PHP, Java, Kotlin, Scala, Groovy, Swift,
  Objective-C, C, C++, C#, F#, Dart, Zig, Ruby, Lua, Elixir, Erlang, Haskell, OCaml, Elm, R, Julia,
  PowerShell, shell scripts and SQL.
- **The web:** HTML, Vue and Svelte components, CSS, SCSS and LESS.
- **Data and configuration:** TOML, YAML (Docker Compose files too), JSON, XML, Protocol Buffers,
  GraphQL, Nix, HCL and Terraform, INI and Java properties.
- **Builds:** Makefiles, CMake and Dockerfiles.

Zed has many of these languages only through their own extensions, PHP, Scala, C#, Elixir, Nix and
Terraform among them. orthra checks such files once that extension is installed.

## Install

Install **Orthra** from Zed's extensions. The first file it checks fetches orthra itself, from
orthra's [releases](https://github.com/koroblabs/orthra/releases): macOS on Apple silicon or Intel,
Linux on x86_64 or arm64. It is kept until a newer release replaces it, and used offline.

An `orthra` on your `PATH` is used instead, so a build of your own wins:

```sh
cargo install --locked --git https://github.com/koroblabs/orthra orthra-cli
```

## Settings

All optional, in Zed's settings:

```jsonc
{
  "lsp": {
    "orthra": {
      // A binary of your own, instead of PATH or the download.
      "binary": { "path": "/path/to/orthra" },
      "initialization_options": {
        // Added to the project's orthra.toml. Built in, English is en-US and en-GB both; naming
        // one variant, as here, checks that one alone.
        "languages": ["en-US", "uk-UA"],
        // error | warning (the default) | information | hint
        "severity": "hint",
        "context": {
          // The built-in model, which reorders a word's corrections by its sentence.
          "rank": true,
          // A model you run, for "Rewrite" and "Rephrase": see below.
          "rewrite": { "model": "qwen2.5:3b", "endpoint": "http://127.0.0.1:11434/v1" },
        },
      },
    },
  },
}
```

### Rewriting with a model you run

With `context.rewrite` set, the code-action menu offers **Rewrite sentence**, and with text selected
**Rewrite selection** and **Rephrase selection**. The model's answer replaces the text once you
choose it. Any server with an OpenAI-compatible `/v1/chat/completions` works:

| server                     | `endpoint`                  |
| -------------------------- | --------------------------- |
| Ollama (the default)       | `http://127.0.0.1:11434/v1` |
| llama.cpp's `llama-server` | `http://127.0.0.1:8080/v1`  |
| LM Studio                  | `http://127.0.0.1:1234/v1`  |

**Rewrite** corrects spelling and grammar and keeps the wording that is right; **Rephrase** says the
same thing more naturally. Neither runs on its own or changes what is reported.

```jsonc
"rewrite": {
  "model": "qwen2.5:3b",
  "apiKey": "...", // only for a server that asks for one
  "prompts": { "rephrase": "Rephrase this more formally{language}. Reply with the text only." },
}
```

A prompt of your own replaces orthra's, and `{language}` in it names the text's language. orthra
refuses an endpoint on another machine unless the setting says `"allowRemote": true`, and the menu
then names the endpoint instead of "local model".

The built-in model (`rank`) and the one you run (`rewrite`) are independent: the first orders a
word's corrections and cannot write, the second writes. Either, both or neither may be on. orthra
0.1.0's `context.enabled` and `context.http` are no longer read, and the server says so.

### Every exact fix on save

A known typo's only correction, and a word typed in two scripts, can be applied across a file each
time Zed formats it on save:

```jsonc
"code_actions_on_format": { "source.fixAll.orthra": true }
```

The same action, **Apply every exact fix in this file**, is in the code-action menu on such a
finding.

A project's own settings, the words it accepts and the files it ignores are in `orthra.toml` and
`.orthra/words.txt`: see [orthra's README](https://github.com/koroblabs/orthra#configure).

## Working on it

In Zed, run **zed: install dev extension** and pick this directory.
