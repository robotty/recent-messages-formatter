# recent-messages-formatter

## Usage

```none
Simple program to fetch a Twitch channel's recent messages and to print all normal messages in a simple readable format.

Usage: recent-messages-formatter <CHANNEL_LOGIN>

Arguments:
  <CHANNEL_LOGIN>  The channel login of the Twitch channel to fetch. E.g. `pajlada` or `riotgames`

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Example output:

```none
$ ./target/release/recent-messages-formatter forsen
[...] a lot of messages, omitted for brevity
drevv08: Aware he is never beating it, is he?
admiralooo5: 99.99% = 0.01% LULE 99.99% = 0.01% LULE 99.99% = 0.01% LULE 99.99% = 0.01% LULE 99.99% = 0.01% LULE 99.99% = 0.01% LULE 99.99% = 0.01% LULE 99.99% = 0.01% LULE
pinkpank0: Sisyfors
bored_guy1: AUGUST 12 2036 Aware AUGUST 12 2036 Aware AUGUST 12 2036 Aware AUGUST 12 2036 Aware
pepegalos: 99.99% = 0.01% LULE 99.99% = 0.01% LULE 99.99% = 0.01% LULE 99.99% = 0.01% LULE 99.99% = 0.01% LULE 99.99% = 0.01% LULE 99.99% = 0.01% LULE 99.99% = 0.01% LULE 󠀀
2dphsssy: forsenInsane
hpozsl: Hit the 0.01%, unlucky forsenClown 󠀀 Hit the 0.01%, unlucky forsenClown 󠀀 Hit the 0.01%, unlucky forsenClown 󠀀 Hit the 0.01%, unlucky forsenClown 󠀀 Hit the 0.01%, unlucky forsenClown 󠀀 Hit the 0.01%, unlucky forsenClown 󠀀
kesilchen: forsenE
LemoorerZ: @ghandyuk forsenLaughingAtYou 🔵 @ghandyuk forsenLaughingAtYou 🔵 @ghandyuk forsenLaughingAtYou 🔵 @ghandyuk forsenLaughingAtYou 🔵 @ghandyuk forsenLaughingAtYou 🔵 @ghandyuk forsenLaughingAtYou 🔵 @ghandyuk forsenLaughingAtYou  🔵
JammyJ0: @forsen with the wither skeletons, obviously you eat RIGHT AFTER you get to safety before you start poking them with an axe. youre not an octopus but you should have a brain....
kalimaman: OLD SENILE MAN PLAYS MINECRAFT OLD SENILE MAN PLAYS MINECRAFT OLD SENILE MAN PLAYS MINECRAFT
```

## Building

1. Install Rust (https://www.rust-lang.org/)
2. `git clone https://github.com/robotty/recent-messages-formatter.git`
3. `cd recent-messages-formatter`
4. `cargo build --release`
5. You can find the built executable in `./target/release/recent-messages-formatter`. Feel free to copy this file to wherever you need it, it does not need any other files around it (It is statically linked).
6. Try it out: `./target/release/recent-messages-formatter pajlada`

This process works on Linux, Mac and Windows. Note that on Windows, `./target/release/recent-messages-formatter` becomes `./target/release/recent-messages-formatter.exe`.
