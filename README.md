# cd2

A feature clone of `autojump` - written in Rust.

Keeps a count of the times you `cd` into a filesystem dir and sends you to most popular directories accordingly.

## Install

(WIP)

- Build and cp the binary to your PATH
  - cargo build --release
  - cp target/release/cd2 <TO_PATH>

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
