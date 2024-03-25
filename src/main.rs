use std::mem;

fn main() {
    // Memory usage of u32 value
    let number_one: u32 = 1;
    let size_of_number_one = mem::size_of_val(&number_one);
    println!("Size of u32 value '1': {} bytes.", size_of_number_one);

    // Memory usage of array of bits representing ASCII '1'
    let ascii_1_bits: [u8; 8] = [0, 0, 1, 1, 0, 0, 0, 1];
    let size_of_ascii_1_bits = mem::size_of_val(&ascii_1_bits);
    println!("Size of array of bits representing ASCII '1': {} bytes.", size_of_ascii_1_bits);

    // Memory usage of array of booleans representing ASCII '1'
    let ascii_1_bools: [bool; 8] = [false, false, true, true, false, false ,false ,true];
    let size_of_ascii_1_bool: usize = mem::size_of_val(&ascii_1_bools);
    println!("Size of array of bools representing ASCII '1': {} bytes.", size_of_ascii_1_bool);

    // Memory usage of a single u8
    let bit_u8: u8 = 1;
    let size_of_u8: usize = mem::size_of_val(&bit_u8);
    println!("Size of a single u8 variable is {} bytes.", size_of_u8);


    // Define the struct
    struct Foo {
        bar: bool,
    }

    // Determine the size of foo
    let size_of_foo: usize = std::mem::size_of::<Foo>();
    println!("Size of foo struct is {} bytes.", size_of_foo);

    struct Foo2
    {
        a: bool,
        b: u8,
    }
    let size_of_foo2: usize = std::mem::size_of::<Foo2>();
    println!("Size of foo2 struct is {} bytes.", size_of_foo2);

    struct Foo3
    {
        a: bool,
        b: u32
    }
    let size_of_foo3: usize = std::mem::size_of::<Foo3>();
    println!("Size of foo3 struct is {} bytes.", size_of_foo3);
}