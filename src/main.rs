use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting)
    }
}

fn main() {
    let visitor_list = [
        Visitor::new("bert", "hell Bert, you're great"),
        Visitor::new("steve", "Hey Steve, how are you?"),
        Visitor::new("fred", "Wow, you're amazing!"),
    ];

    println!("Hello, what's your name?");

    let name = what_is_your_name();

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You're not allowed!"),
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("failed to read line");

    your_name.trim().to_lowercase()
}
