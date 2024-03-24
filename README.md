# Memory Usage Comparison in Rust

This Rust program demonstrates the memory usage of different data representations for the value '1'.

## Description

The program compares the memory usage of the following representations of the value '1':

1. `u32` value representing '1'.
2. Array of bits representing the ASCII character '1'.
3. Array of booleans representing the ASCII character '1'.
4. Single `u8` value.

The program prints out the size of each representation in bytes.

## Program Execution

To run the program:

1. Ensure you have [Rust installed](https://www.rust-lang.org/tools/install) on your system.
2. Copy the project into your local `git clone https://github.com/markheramis/rust-memory-usage-experiment.git`
3. Open your terminal and navigate to this project's root directory.
4. Build the project `cargo build`.
5. Run the project `cargo run`.

## Results

The program output provides insights into the memory usage of different data representations for the value '1'.

```bash
Size of u32 value '1': 4 bytes
Size of array of bits representing ASCII '1': 8 bytes
Size of array of bools representing ASCII '1': 8 bytes
Size of a single u8 variable is 1 bytes
```
## Author

This program was created by Mark Heramis.

## Why?

I was just curious and why not?

## License

This project is licensed under the [MIT License](LICENSE).
