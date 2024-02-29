# ttml2srt

🔧 a simple tool to convert subtitles in the TTML format over to more mainstream SRT format

![alt Experimental](https://img.shields.io/badge/Type-Tools-red.svg)
![alt Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![alt Binary](https://img.shields.io/badge/Architecture-binary-green.svg)
![alt Failed](https://img.shields.io/badge/Failed-👎_0-red.svg)
![alt Passed](https://img.shields.io/badge/Passed-👍_0-green.svg)
![alt Version](https://img.shields.io/badge/version-0.1.0_BETA-blue.svg)

```console

_   _              _  _____          _
| | | |           | |/ __  \        | |
| |_| |_ _ __ ___ | |`' / /'___ _ __| |_
| __| __| '_ ` _ \| |  / / / __| '__| __|
| |_| |_| | | | | | |./ /__\__ \ |  | |_
 \__|\__|_| |_| |_|_|\_____/___/_|   \__|


```

# Introduction

The BBC _(British Broadcasting Corporation)_ and perhaps others tend to use the rather obscure subtitle format known as `ttml` [Timed Text Markup Language](https://en.wikipedia.org/wiki/Timed_Text_Markup_Language) . Most video players and software tend to use the much more mainstream format called `srt`.

For a while I've been using [`ttconv`](https://github.com/sandflow/ttconv) written by `sandflow` in Python. While I'm sure that covers vastly more edge cases and what not, installing and setting it up is a chore! and it's weirdly slow _(it's Python so maybe not entirely a surprise)_.

I'm a simple man with simple needs and so `ttml2srt` was born, it's a single binary with a single purpose take `ttml` and spit out `srt` and do it quickly.

## How to build from source

Just use the standard Rust `cargo` toolchain to build from source. Alternatively to build a statically
linked binary, just run the handy build script `./build.sh`.

```console
$ cargo build
Finished dev [unoptimized + debuginfo] target(s) in 0.01s
    Running `target/debug/ttml2srt`
....
```

## How to use

```console
Anhar Hussain Miah

_   _              _  _____          _
| | | |           | |/ __  \        | |
| |_| |_ _ __ ___ | |`' / /'___ _ __| |_
| __| __| '_ ` _ \| |  / / / __| '__| __|
| |_| |_| | | | | | |./ /__\__ \ |  | |_
 \__|\__|_| |_| |_|_|\_____/___/_|   \__|

        2024 (c) TTML2SRT v0.1.0

USAGE:
    ttml2srt --file-name <FILE_NAME>

OPTIONS:
    -f, --file-name <FILE_NAME>    the TTML filename e.g ttml2srt -f subtitle.ttml
    -h, --help                     Print help information
    -V, --version                  Print version information
```

# Version

0.1.0-BETA

## Contributors

- [anharmiah](https://github.com/anharhussainmiah) Anhar Miah - creator, maintainer
