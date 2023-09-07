#![allow(overflowing_literals)]

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    let decimal = 65.4321_f32;

    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as u16 is: {}", 1000 as u16);

    println!("1000 as u8 is: {}", 1000 as u8);

    let x: u8 = 1;
    let y = 2u32;
    let z: f32 = 3.0;

    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    let elem = 5u8;

    let mut vec = Vec::new();

    vec.push(elem);

    println!("{:?}", vec);

    let nano: NanoSecond = 5 as U64;

    println!("Nano second: {}", nano);

    let inch: Inch = 2;

    println!("Inch: {}", inch);
}
