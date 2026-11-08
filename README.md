# orthra for Zed

[orthra](https://github.com/koroblabs/orthra) is a spell-checking language server that is fast and
quiet. It checks prose, and the comments, strings and declared names in code: Rust, TypeScript,
JavaScript, Python, Go, PHP, Java, Kotlin, Swift, C, C++, Ruby, Lua, shell scripts, SQL, HTML, CSS,
SCSS, LESS, Markdown, TOML, YAML, JSON, plain text and commit messages. A word your project
declares, in its manifest or in its code, is a word, and it runs entirely on your machine.

Zed has some of these languages only through their own extensions: PHP, TOML, HTML, SCSS, LESS, SQL,
Lua, Java, Kotlin, Swift and Ruby. orthra checks such files once that extension is installed.

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
          "enabled": true,
          // A model you run, for "Rewrite sentence" and "Rewrite selection": see below.
          "http": { "model": "qwen2.5:3b", "endpoint": "http://127.0.0.1:11434/v1" },
        },
      },
    },
  },
}
```

### Rewriting with a model you run

With `context.http` set, the code-action menu offers **Rewrite sentence**, or **Rewrite selection**
with text selected, and the model's answer replaces the text once you choose it. Any server with an
OpenAI-compatible `/v1/chat/completions` works:

| server                     | `endpoint`                  |
| -------------------------- | --------------------------- |
| Ollama (the default)       | `http://127.0.0.1:11434/v1` |
| llama.cpp's `llama-server` | `http://127.0.0.1:8080/v1`  |
| LM Studio                  | `http://127.0.0.1:1234/v1`  |

The model corrects spelling and grammar and keeps the wording that is right. It never runs on its
own and never changes what is reported. orthra refuses an endpoint on another machine unless the
setting says `"allowRemote": true`, and the menu then names the endpoint instead of "local model".
No API key is sent.

A project's own settings, the words it accepts and the files it ignores are in `orthra.toml` and
`.orthra/words.txt`: see [orthra's README](https://github.com/koroblabs/orthra#configure).

## Working on it

In Zed, run **zed: install dev extension** and pick this directory.
