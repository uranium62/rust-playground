#[allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String
}

impl Person {
    fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into()
        }
    }
}

fn main() {
    let person1 = Person::new("Alex");
    let person2 = Person::new("Alex".to_string());

    println!("{:?}", person1);
    println!("{:?}", person2);
}
