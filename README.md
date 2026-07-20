# mermaid-ascii

Turn [Mermaid](https://mermaid.js.org/) diagrams into clean ASCII art for your terminal.

```sh
printf 'flowchart LR\nA[Source] --> B[Render] --> C[Terminal]\n' | mermaid-ascii
```

```

┌────────┐    ┌────────┐    ┌──────────┐
│ Source ├───▶│ Render ├───▶│ Terminal │
└────────┘    └────────┘    └──────────┘
```

Reads a file or stdin, writes plain text to stdout. Handles flowchart, sequence,
state, class, and entity-relationship diagrams. Anything else is printed as-is in
a framed box.

## Install

```sh
cargo install --path .
```

For a standalone binary with no system dependencies:

```sh
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
# binary: target/x86_64-unknown-linux-musl/release/mermaid-ascii
```

## Usage

```sh
mermaid-ascii diagram.mmd          # from a file
cat diagram.mmd | mermaid-ascii    # from stdin
mermaid-ascii --width 80 d.mmd     # cap the output width
```

Ready-made diagrams to try live in [`examples/`](examples/):

```sh
mermaid-ascii examples/flowchart.mmd
```

## Library

`mermaid-ascii` is also a Rust library. `render` returns plain Unicode lines for a
CLI, or styled [`ratatui`](https://ratatui.rs/) lines for a TUI.

## Why

Inspired by Simon Willison's [_Grok 4 can render Mermaid diagrams as ASCII art_](https://simonwillison.net/2026/Jul/16/grok-mermaid/).

Coding agents write Mermaid well but draw ASCII art badly. Let them write Mermaid
and have this render it.

## Credits

- Forked from the renderer in [xAI's `grok-build`](https://github.com/xai-org/grok-build/blob/c68e39f60462f28d9be5e683d9cbe2c57b1a5027/crates/codegen/xai-grok-markdown/src/mermaid.rs) (Apache-2.0). Thanks to the xAI team.
- Example diagrams from Simon Willison's [`tools`](https://github.com/simonw/tools/blob/cfa72d42d7c7a06d9ad62ee8fdcea16ba634db7d/grok-mermaid.html#L141-L191) (Apache-2.0).

## What this fork changes

Starting from grok-build's renderer at commit [`c68e39f`](https://github.com/xai-org/grok-build/tree/c68e39f60462f28d9be5e683d9cbe2c57b1a5027), and building on top of it, this fork:

- pulls the renderer out into a standalone library plus a small file/stdin CLI (grok-build embeds it in a larger markdown system);
- hardens it for untrusted input by stripping terminal control characters, capping input size, and bounding the parsing and layout work;
- adds a static x86-64 Linux (musl) build;
- improves routing of long edges so connectors stay inside the diagram instead of looping around the outside;
- drops the grok-build parts it doesn't need: markdown streaming, the pager UI, PNG rendering, and settings.

## License

Apache-2.0. See [`LICENSE`](LICENSE).
