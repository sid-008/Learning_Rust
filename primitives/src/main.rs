use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element is {}", slice[0]);
    println!("Length of slice is {}", slice.len());
}

fn main() {
    /*
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_int = 5i32; // suffix annotation this is equivalent to an_int: i32

    let def_float = 3.0; // this uses the default f64
    let def_int = 12; // this uses i32

    let mut inferred_type = 12; // type i64 is inferred here from the below line
    inferred_type = 23234i64;

    let mut mutable = 12; // mutable i32
    mutable = 21;

    // mutable = true; will result in as error

    // however variables can be overwritten with the use of let. This is called shadowing
    let mutable = true;
    */

    println!("1+2={}", 1u32 + 2);
    println!("1-2={}", 1i32 - 2);

    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("One million is written as {}", 1_000_000u32);

    // tuples are in ()
    let pair = (1, true);
    println!("{:?}", pair);

    let tuple = (1, "hello", 4.5, true);
    println!("{:#?}", tuple);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    println!(
        "From tuple, 2nd member is: {0}, the 3rd member is {1}, the second member again is {0}",
        tuple.1, tuple.2
    );

    println!(
        "From tuple again, {hello} is hello, 1 is {one}",
        hello = tuple.1,
        one = tuple.0,
    );

    // arrays are in [], they are fixed size and have type signature [T; length]
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", xs); // fixed size array

    println!("{}", xs[1]);
    let ys: [i32; 500] = [0; 500];
    println!("{:?}", ys[0]);

    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    analyze_slice(&xs); // borrowing xs

    println!("{:?}", &xs[1..4]); // borrowing a part of xs

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
}
