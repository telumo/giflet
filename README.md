# giflet [![Rust](https://github.com/telumo/giflet/workflows/Rust/badge.svg)](https://github.com/telumo/giflet/actions)

Cli tool to create GIF easily!!

## Overview

T.B.D.

## Screenshot

![output](output.gif)

## Usage

    $ giflet [OPTIONS] <directory>

## Options

- **-o**, **--output**: output file name
- **-d**, **--delay**: delay time
- **-w**, **--width**: output file width
- **-h**, **--height**: output file height

## Installation

### Cargo Install

T.B.D

### Homebrew

If you're using [homebrew](https://brew.sh/), you can use the `brew install` command:

    $ brew tap telumo/giflet

    $ brew install giflet

[Formula](https://github.com/telumo/homebrew-giflet/blob/master/Formula/giflet.rb)

### Binary download
```sh
$ wget https://github.com/telumo/giflet/releases/download/v0.1.3-alpha/giflet-0.3.0-x86_64-linux-musl.tar.gz

$ sha256sum giflet-0.3.0-x86_64-linux-musl.tar.gz
# => 085105653aa2d868f8cfa77ba8f7ff13d34c1ab652bc3ea4817cd436e71e98be  giflet-0.3.0-x86_64-linux-musl.tar.gz

$ tar -x -f giflet-0.3.0-x86_64-linux-musl.tar.gz -z

$ ./giflet ./images

```
## Testing

T.B.D
