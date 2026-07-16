# mermaid-ascii

Render Mermaid source as terminal-safe Unicode art. Reads one file or stdin and writes plain text to stdout.

```sh
printf 'flowchart LR\nA[Source] --> B[Output]\n' | mermaid-ascii --width 80
mermaid-ascii diagram.mmd
```

Supported: flowchart, sequence, state, class, and entity-relationship diagrams. Unsupported syntax is shown in a framed source fallback.

## Examples

[`examples/`](examples/) contains the [grok-mermaid](https://tools.simonwillison.net/grok-mermaid) examples. Render one with `mermaid-ascii examples/flowchart.mmd`.

## Build

The checked-in Cargo config targets static x86-64 Linux with musl:

```sh
rustup target add x86_64-unknown-linux-musl
cargo build --release
```

Binary: `target/x86_64-unknown-linux-musl/release/mermaid-ascii`.

## Library

`render` also returns styled `ratatui::text::Line` values for TUI use and plain Unicode lines for CLI use.

## Acknowledgment

Terminal renderer adapted from [xAI's `grok-build`](https://github.com/xai-org/grok-build/tree/c68e39f60462f28d9be5e683d9cbe2c57b1a5027), licensed under Apache-2.0.
