fn main() {
    let x = 2;
    let y: i8 = 2;

    println!("Hello, world! {} times", x * y);
    let name = String::from("Mr. Meeseeks");
    println!("My name is {}", name);
    let mut hisname = name;
    println!("His name is {}", hisname);
    //println!("I said my name is {}", name);
    hisname = format!("{}new", hisname);
    println!("His name is {}", hisname);

    let name = Box::new(String::from("Fred Willard"));
    println!("name is {}", name);

    let mut foo = stringify!(what a deal);
    let bar = &mut foo;
    *bar = stringify!(is it though?);
    //println!("{}", foo);
    println!("{}", bar);
}
