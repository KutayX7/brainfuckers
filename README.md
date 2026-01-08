# brainfuckers
A simple Brainfuck interpreter

Crate: https://crates.io/crates/brainfuckers

## Usage
If you supply a filename as the first argument, it will be executed.
If no filename has been provided, the first line will be considered as code and will be executed.

## Compatibility
* Each cell has a value between 0-255 and initialized to 0
* Cell values wrap around
* There are infinite amount of cells in both directions
* Only the 8 primary Brainfuck operations are handled, anything else is noop
* Cell set to 0 on EOF
* EOF is 0x00
