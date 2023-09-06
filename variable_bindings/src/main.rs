fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_int = an_integer;

    println!("An integer: {:?}", copied_int);
    println!("A boolean: {:?}", a_boolean);
    println!("A unit value: {:?}", unit);

    let _unused_silent_var = 3u32;

    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // _immutable_binding += 1;

    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("Inner short lived binding: {}", short_lived_binding);
    }

    // println!("Outer short: {}", short_lived_binding);

    println!("Outer long: {}", long_lived_binding);

    let shadowed_binding = 1;

    {
        println!("Before being shadowed: {}", shadowed_binding);

        let shadowed_binding = "abc";

        println!("shadowed_binding in inner block: {}", shadowed_binding);
    }

    println!("Outside inner block: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("Shadowed in outer block: {}", shadowed_binding);

    let a_binding;

    {
        let x = 2;

        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // println!("Another binding: {}", another_binding);

    another_binding = 1;
    println!("Another binding now: {}", another_binding);

    let mut _mutable_integer = 7i32;
    {
        // shadowing by immutable
        let _mutable_integer = _mutable_integer;

        // _mutable_integer = 50;
    }

    println!("Pre mutation: {}", _mutable_integer);
    _mutable_integer = 3;
    println!("Mutable integer non shadowed is {}", _mutable_integer);
}
