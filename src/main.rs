use std::io::stdin;

fn main() {
    let visitor_list = ["bert", "steve", "fred"];
    let mut allow_them_in = false;

    println!("Hello, what's your name?");

    let name = what_is_your_name();

    for visitor in &visitor_list {
        if visitor == &name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Hello, {:?}", name)
    } else {
        println!("You're not allowed")
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("failed to read line");

    your_name.trim().to_lowercase()
}
