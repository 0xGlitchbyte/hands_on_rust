use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, greeting: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probabtionary member", self.name),
            &VisitorAction::Refuse => println!("Do not allow {} in.", self.name),
        }
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}
fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line.");
    return your_name.trim().to_lowercase();
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new(
            "bert",
            "Hello Bert, enjoy your treehouse.",
            VisitorAction::Accept,
            45,
        ),
        Visitor::new(
            "steve",
            "Hi Steve. Your milk is in the fridge.",
            VisitorAction::AcceptWithNote {
                note: String::from(" Lactose-Intolerant."),
            },
            15,
        ),
        Visitor::new("fred", "Wow, who invited Fred?", VisitorAction::Refuse, 30),
    ];
    loop {
        println!("Hello, what's your name?");
        let name = what_is_your_name();
        //let mut allowed_in = false;
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(
                        &name,
                        "New Friend",
                        VisitorAction::Probation,
                        0,
                    ));
                }
            }
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list)
}
