#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    // classic C struct
    name: String,
    age: u8,
}

struct Unit; // fieldless struct

struct Pair(i32, f32); //tuple struct

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed {}", c),
        WebEvent::Paste(s) => println!("Pasted {}", s),
        WebEvent::Click { x, y } => {
            println!("click at x = {} and y = {}", x, y);
        }
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("Point coordinates are: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };

    println!("Second point is: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("Pair contains {} and {}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("Pair now contains {:?} and {:?}", integer, decimal);

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // explicit 'use' each name so no need of manual scoping i.e specifying enum name
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Poor => println!("The poor have no money"),
        Rich => println!("The rich have a lot of money"),
    }

    match work {
        Civilian => println!("Civilians work"),
        Soldier => println!("Soliders fight"),
    }
}
