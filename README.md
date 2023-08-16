# recent-messages-formatter

## Usage

```
Simple program to fetch a Twitch channel's recent messages and to print all normal messages in a simple readable format.

Usage: recent-messages-formatter <CHANNEL_LOGIN>

Arguments:
  <CHANNEL_LOGIN>  The channel login of the Twitch channel to fetch. E.g. `pajlada` or `riotgames`

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Building

1. Install Rust (https://www.rust-lang.org/)
2. `git clone https://github.com/robotty/recent-messages-formatter.git`
3. `cd recent-messages-formatter`
4. `cargo build --release`
5. You can find the built executable in `./target/release/recent-messages-formatter`. Feel free to copy this file to wherever you need it, it does not need any other files around it (It is statically linked).
6. Try it out: `./target/release/recent-messages-formatter pajlada`

This process works on Linux, Mac and Windows. Note that on Windows, `./target/release/recent-messages-formatter` becomes `./target/release/recent-messages-formatter.exe`.
