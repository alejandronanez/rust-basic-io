use std::io::stdin;

#[derive(Debug)]
enum VisitorActions {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorActions,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorActions, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", VisitorActions::Accept, 30),
        Visitor::new("steve", VisitorActions::Refuse, 23),
        Visitor::new(
            "fred",
            VisitorActions::AcceptWithNote {
                note: "An young boy".to_string(),
            },
            15,
        ),
    ];
    loop {
        println!("Hello, what's your name?");

        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => match &visitor.action {
                VisitorActions::Accept => println!("Welcome to the club, {}", visitor.name),
                VisitorActions::AcceptWithNote { note } => {
                    println!("Welcome to the club, {}", visitor.name);
                    println!("{}", note);

                    if visitor.age < 21 {
                        println!("Don't serve alcohol to {}", visitor.name);
                    }
                }
                VisitorActions::Probation => {
                    println!("{}, is now a probationary member", visitor.name)
                }
                VisitorActions::Refuse => println!("Don't allow {} in", visitor.name),
            },
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor's list", name);
                    visitor_list.push(Visitor::new(&name, VisitorActions::Probation, 0));
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
