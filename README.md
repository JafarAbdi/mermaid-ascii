# mermaid-ascii

Render Mermaid source as terminal-safe Unicode art. Reads one file or stdin and writes plain text to stdout.

```sh
printf 'flowchart LR\nA[Source] --> B[Output]\n' | mermaid-ascii --width 80
mermaid-ascii diagram.mmd
```

Supported: flowchart, sequence, state, class, and entity-relationship diagrams. Unsupported syntax is shown in a framed source fallback.

## Examples

[`examples/`](examples/) includes the examples from Simon Willison's [grok-mermaid](https://github.com/simonw/tools/blob/cfa72d42d7c7a06d9ad62ee8fdcea16ba634db7d/grok-mermaid.html#L141-L191), used under its [Apache-2.0 license](https://github.com/simonw/tools/blob/cfa72d42d7c7a06d9ad62ee8fdcea16ba634db7d/LICENSE). Render one with `mermaid-ascii examples/flowchart.mmd`.

## Build

Build for the host with `cargo build --release`.

For a static x86-64 Linux binary, install `musl-gcc`, then run:

```sh
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

Binary: `target/x86_64-unknown-linux-musl/release/mermaid-ascii`.

## Library

`render` also returns styled `ratatui::text::Line` values for TUI use and plain Unicode lines for CLI use.

## Origin and differences

Terminal renderer adapted from [xAI's `grok-build`](https://github.com/xai-org/grok-build/blob/c68e39f60462f28d9be5e683d9cbe2c57b1a5027/crates/codegen/xai-grok-markdown/src/mermaid.rs), used under its [Apache-2.0 license](https://github.com/xai-org/grok-build/blob/c68e39f60462f28d9be5e683d9cbe2c57b1a5027/LICENSE).

Compared with that revision, this package:

- extracts the terminal renderer into a standalone library and stdin/file CLI;
- sanitizes terminal control characters, caps CLI input, and bounds parsing, layout, and fallback output;
- supports native builds plus an explicit static x86-64 musl build;
- normalizes long forward edges with bounded, invisible Sugiyama routing points instead of exterior back-edge lanes;
- excludes the markdown streaming, pager UI, PNG worker/cache, and settings integration.
