#[derive(Debug)]
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
    let p = Point(p1.0 + p2.0, p1.1 + p2.1);
    println!("&p.0: {:p}", &p.0);
    p
}

//In C++, copy elision has to be defined in the language specification because constructors can have side effects.
//In Rust, this is not an issue at all. If RVO did not happen, Rust will always performs a simple and efficient memcpy copy.

fn main() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("&p3.0: {:p}", &p3.0);//in debug mode address of p in add function and p3 in main is diferent, but in production same
    println!("{p1:?} + {p2:?} = {p3:?}");
}