fn main() {
    let x = fun() + fun() + fun();

    println!("Hello, world! x is {}", x);
}

fn fun() -> i8 {
    return 42;
}
