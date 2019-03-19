# Chip-8

CHIP-8 Virtual Machine implementation, built in Rust.

A small project because I wanted to grok emulators and almost every (if not
all) the sources agreed that implementing CHIP-8 was the starting point for it.

I also wanted to grasp the basics of Rust as a systems language, so after
reading the first 13 chapters of [The Rust Book][The Rust Programming
Language], I found a [guide][How to write an emulator (CHIP-8 interpreter)] and
started hacking on it.

I used Cowgod's [excellent reference][Cowgod's Chip-8 Reference] as well as
[Wikipedia][CHIP-8 Page on Wikipedia] for the definition and details of the
opcodes, and stole the idea of the ProgramCounter as a type from [Starr
Horne][Starr Horne's implementation] because I thought it was an elegant solution.

In theory, `src/lib.rs` contains all the functionality to work as the VM, with
it's only dependency being the `rand` crate. That code could work as a library
you would be able to call (through its APIs) if you wanted to make your own
implementation.

For example, if instead of using SDL2 for the display, I wanted to use the
terminal, I could write `src/terminal_display.rs` using a terminal crate, pass
to it the `Chip8.screen` variable, and `src/terminal_display.rs` would do all
the heavy lifting to display the Chip8 screen data. It would be interesting to
return to this in the future.

## Usage

Make sure you fulfill the [Requirements](#requirements) before attempting to build
the binary.

1. Clone this repository
2. Change the working directory to the root of the repo
3. `cargo run path/to/game`
4. Leave a comment, a suggestion or report a bug. I would be happy to talk
   about this!

### Controls

The original CHIP-8 had a physical keypad with 16 keys that went from 0 to F.
This implementation maps the standard QWERTY keyboard to the CHIP-8 keypad like
this:

```
 QWERTY Keyboard       CHIP-8 Keypad

| 1 | 2 | 3 | 4 |    | 1 | 2 | 3 | C |
| Q | W | E | R | -> | 4 | 5 | 6 | D |
| A | S | D | F |    | 7 | 8 | 9 | E |
| Z | X | C | V |    | A | 0 | B | F |
```

## Requirements

The emulator uses the `sdl2` crate, which depends on `libsdl2`. To compile
`sdl2` you need `libsdl2` with it's headers. On Debian you can run:

```sh
sudo apt-get install libsdl2-dev
```

## Testing

You can test this project as a standard Rust project using the `cargo` tool:
```sh
cargo test
```
Or trying the binary with some [public domain games][Zophar's Domain Public Domain ROMs]:
```sh
cargo run <game>
```

## License

This project is distributed under the terms of the [GNU General Public License][GPL].

## References

* [How to write an emulator (CHIP-8 interpreter)]
* [Cowgod's Chip-8 Reference]
* [CHIP-8 Page on Wikipedia]
* [Starr Horne's implementation]
* [Zophar's Domain Public Domain ROMs]

[How to write an emulator (CHIP-8 interpreter)]: http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
[Cowgod's Chip-8 Reference]: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
[CHIP-8 Page on Wikipedia]: https://en.wikipedia.org/wiki/CHIP-8
[Starr Horne's implementation]: https://github.com/starrhorne/chip8-rust
[Zophar's Domain Public Domain ROMs]: https://www.zophar.net/pdroms.html
[GPL]: https://www.gnu.org/licenses/gpl.html

[Ruby Quiz 88: Chip-8 Emulator]: https://rubyquiz.com/quiz88.html
[The Rust Programming Language]: https://doc.rust-lang.org/book/index.html
