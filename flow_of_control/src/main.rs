fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let mut count: u32 = 0;

    println!("Count till 5");
    loop {
        count += 1;
        println!("Count is: {}", count);

        if count == 5 {
            println!("Break!");
            break;
        }
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
    println!("Counter {}", counter);

    let names = vec!["Bob", "Ferris", "sid"];

    for name in names.iter() {
        //iter borrows each element of the collection leaving the names
        //vector untouched
        match name {
            &"Ferris" => println!("There is a rustacean amogus"),
            _ => println!("Hello {}", name),
        }
    }

    println!("Names: {:?}", names);

    // for i in 1..=100 {
    //     println!("{}", i);
    // }

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("Names: {:?}", names);

    let number = 19;
    println!("Tell me about {}", number);

    match number {
        1 => println!("This is 1"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Nothing special"),
    }
}
