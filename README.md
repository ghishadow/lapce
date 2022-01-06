<h1 align="center">
  <img src="extra/images/logo.png" width=200 height=200/><br>
  Lapce
</h1>

<h4 align="center">Lightning-fast and Powerful Code Editor written in Rust</h4>

[![chat](https://img.shields.io/badge/zulip-join_chat-brightgreen.svg)](https://lapce.zulipchat.com)

[![Release](https://github.com/ghishadow/lapce/actions/workflows/release.yml/badge.svg)](https://github.com/ghishadow/lapce/actions/workflows/release.yml)

Lapce is written in pure Rust, with UI in [Druid](https://github.com/linebender/druid). It's using [Xi-Editor](https://github.com/xi-editor/xi-editor)'s [Rope Science](https://xi-editor.io/docs/rope_science_00.html) for text editing, and using [Wgpu](https://github.com/gfx-rs/wgpu) for rendering. 

![](https://github.com/lapce/lapce/blob/master/extra/images/screenshot.png?raw=true)

## Features

* Modal Editing (Vim like) support as first class citizen (can be turned off as well)
* Built in LSP support
* Built in remote development support (inspired by [VSCode Remote Development](https://code.visualstudio.com/docs/remote/remote-overview))
* Plugin can be written in programming languages that can compile to [WASI](https://wasi.dev/) (C, Rust, [AssemblyScript](https://www.assemblyscript.org/))
* Built in terminal

## Build from source

### Install the Rust compiler with `rustup`

1. Install [`rustup.rs`](https://rustup.rs/).

### Dependencies
#### Ubuntu
```sh
sudo apt-get install cmake pkg-config libfreetype6-dev libfontconfig1-dev libxcb-xfixes0-dev libxkbcommon-dev
```
### Building
```sh
cargo build --release
```
The exectuable will be available at `target/release/lapce`

## Feedback

* Chat on [Zulip](https://lapce.zulipchat.com)
* Join the discussion on [Reddit](https://www.reddit.com/r/lapce/)
