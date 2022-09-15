use std::io::stdin;

#[derive(Debug)]
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
    let mut visitor_list = vec![
        Visitor::new("bert", "hell Bert, you're great"),
        Visitor::new("steve", "Hey Steve, how are you?"),
        Visitor::new("fred", "Wow, you're amazing!"),
    ];
    loop {
        println!("Hello, what's your name?");

        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor's list", name);
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
    }
    println!("The final visitor list:\n{:#?}", visitor_list);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("failed to read line");

    your_name.trim().to_lowercase()
}
