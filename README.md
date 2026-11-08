# orthra for Zed

[orthra](https://github.com/koroblabs/orthra) is a spell-checking language server that is fast and
quiet. It checks prose, and the comments, strings and declared names in code: Rust, TypeScript,
Python, Go, PHP, Markdown, TOML, YAML, JSON, CSS, plain text and commit messages. A word your project
declares, in its manifest or in its code, is a word, and it runs entirely on your machine.

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
        // Added to the project's orthra.toml. en-US when neither says.
        "languages": ["en-US", "uk-UA"],
        // error | warning (the default) | information | hint
        "severity": "hint",
        // The optional model, which reorders a word's corrections by its sentence.
        "context": { "enabled": true },
      },
    },
  },
}
```

A project's own settings, the words it accepts and the files it ignores are in `orthra.toml` and
`.orthra/words.txt`: see [orthra's README](https://github.com/koroblabs/orthra#configure).

## Working on it

In Zed, run **zed: install dev extension** and pick this directory.
