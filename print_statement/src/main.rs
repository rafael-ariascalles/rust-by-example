fn main() {
    println!("Hello, world!");

    let name: &str = "poule";
    let last_name: &str = "show";
    println!("{name:#<10} and {last_name:#>10}");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name: &str = "Peter";
    let age: u8 = 27;
    let peter = Person { name, age };
    // Pretty print
    println!("{:?}", peter);
}