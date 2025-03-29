# Riichi Hand Cli

Cli version of [riichi-hand-rs](https://github.com/m4tx/riichi-hand-rs)

---
riichi-hand-cli is a command-line tool for generating mahjong hand images. It can either save the image to a file or copy it directly to your clipboard, making it convenient for game reviews or studying mahjong books without switching between browser windows.

# Installation
```shell
cargo install riichi-hand-cli
```

# Usage
```txt
Cli wrapper of riichi-hand

Usage: riichi-hand-cli [OPTIONS] <HAND>

Arguments:
  <HAND>  Mahjong hand in human-readable format --help for more information

Options:
  -n, --name <NAME>  Name and path of the image to save. If not specified, the image will be copied to clipboard
  -t, --tile <TILE>  Tile design to use [default: yellow]
  -i, --interactive  Interactive mode Can generate multiple images in the same session
  -h, --help         Print help (see more with '--help')
  -V, --version      Print version
```

```shell
# For more detailed information about HAND format
riichi-hand-cli --help
```

# example

```shell
# interactive mode
riichi-hand-cli
# simple one hand
riichi-hand-cli --name hand.png --tile yellow 123456789s567p99m
# short hand command
riichi-hand-cli -n hand.png -t yellow 123456789s567p99m
# some complex example
riichi-hand-cli -n hand.png '123456s99m_5*67p_?99s?'
```

# Attribution

This project uses modified
[riichi-mahjong-tiles](https://github.com/FluffyStuff/riichi-mahjong-tiles)
by [FluffyStuff](https://github.com/FluffyStuff) in
[public domain/CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/).

This project uses mahjong tiles by
[Martin Persson](https://www.martinpersson.org/) which are free for personal
and commercial use under the condition that a link to the author's page
is provided.
