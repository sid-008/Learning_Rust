#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("Hello, world!");

    println!("{} days", 31);
    println!("{0} ipsum {1}, {0} ipsum and {1}", "Lorem", "dolor");

    println!("Base 10: {}", 69420);
    println!("Base 2 (binary): {:b}", 69420);
    println!("Base 8 (octal): {:o}", 69420);
    println!("Base 16 (hexa): {:X}", 69420);

    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    println!("{:?} months in a year.", 12);

    println!("Now {:?} will print", Structure(3));
    println!("Now this will print {:?}", Deep(Structure(7)));
}
