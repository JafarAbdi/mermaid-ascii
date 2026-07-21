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

Reads a file or stdin, writes plain text to stdout. Supported diagrams: flowchart,
sequence, state, class, and entity-relationship. Anything else is shown as-is in a
framed box.

## Install

Download a prebuilt x86-64 Linux binary (glibc or static musl) from the
[releases page](https://github.com/JafarAbdi/mermaid-ascii/releases), or build from
source with `cargo install --path .`.

## Usage

```sh
mermaid-ascii diagram.mmd              # from a file
cat diagram.mmd | mermaid-ascii        # from stdin
mermaid-ascii --width 80 diagram.mmd   # cap the output width
mermaid-ascii examples/flowchart.mmd   # try a bundled example
```

## Why

Inspired by Simon Willison's [_Grok 4 can render Mermaid diagrams as ASCII art_](https://simonwillison.net/2026/Jul/16/grok-mermaid/).

Coding agents write Mermaid well but draw ASCII art badly. Let them write Mermaid
and have this render it.

## Credits

A fork of the terminal renderer in [xAI's `grok-build`](https://github.com/xai-org/grok-build/blob/c68e39f60462f28d9be5e683d9cbe2c57b1a5027/crates/codegen/xai-grok-markdown/src/mermaid.rs)
(Apache-2.0), extracted into a standalone library and CLI and hardened for untrusted
input. Example diagrams from Simon Willison's [`tools`](https://github.com/simonw/tools/blob/cfa72d42d7c7a06d9ad62ee8fdcea16ba634db7d/grok-mermaid.html#L141-L191)
(Apache-2.0). Licensed under Apache-2.0; see [`LICENSE`](LICENSE).

<details>
<summary>What this fork changes</summary>

Starting from grok-build's renderer at commit [`c68e39f`](https://github.com/xai-org/grok-build/tree/c68e39f60462f28d9be5e683d9cbe2c57b1a5027), this fork:

- pulls the renderer out into a standalone library plus a file/stdin CLI;
- hardens it for untrusted input: strips control characters, caps input size, bounds the layout work;
- adds a static x86-64 Linux (musl) build;
- improves long-edge routing so connectors stay inside the diagram instead of looping around the outside;
- drops the grok-build parts it doesn't need: markdown streaming, pager UI, PNG rendering, and settings.
</details>
