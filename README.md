# cd2

A feature clone of [autojump](https://github.com/wting/autojump) - written in Rust.

Keeps a count of the times you `cd` into a filesystem dir and sends you to most popular directories accordingly.

## Install

- Download the binary from [Releases](https://github.com/skyward-luke/cd2/releases)
- Add the following to your shell .rc file, like .bashrc:

```bash
# name the function whatever you like
# which you'll use instead of cd command
function cc {
  res=$(cd2 "$1")
  cd $res
  echo $res
}
```

- Source .rc file: `source ~/.bashrc`

- Use the new function:
  - `cc path/to/my/proj`
  - `cc ~`
  - `cc proj`

## Build locally

Build and cp the binary to your PATH

- `cargo build --release`
- `cp target/release/cd2 <TO_PATH>`
