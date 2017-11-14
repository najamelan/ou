# ou (other user)

**This is a hack that comes without warranty or support. Shared for educational purposes only unless someone wants to whip it up and file pull requests.**

ou allows you to run a command as another user when you are unprivileged by giving the password for the other user. It is like su, but it doesn't need to be run in a terminal. I use it to create a .desktop launcher that people can click to temporarily unlock some things on a public computer by providing a password for a different user.

This only makes sense in an interactive session, so it asks the password through zenity.

## Dependencies

- zenity

## Installation

1. [Install Rust](https://www.rust-lang.org/en-US/install.html)

```bash
git clone https://github.com/najamelan/ou
cd ou
cargo build --release                     # This step also needs internet connection
cp target/release/ou /usr/local/bin
sudo chown root:root /usr/local/bin/ou
sudo chmod u+s       /usr/local/bin/ou
```

Alternatively you can download a binary from the [release](https://github.com/najamelan/ou/releases) page of this repository for linux x64.

## Usage

`ou user cmd`
