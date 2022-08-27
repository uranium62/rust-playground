enum AnimalType {
    Dog,
    Cat
}

struct Dog {}
struct Cat {}

trait Animal {
    fn name(&self) -> &'static str;
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat"
    }
}

fn main() {
    let animal1 = get_animal(AnimalType::Cat);
    println!("{}", animal1.name());
    
    let animal2 = get_animal(AnimalType::Dog);
    println!("{}", animal2.name());
    
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog{}),
        Box::new(Dog{}),
        Box::new(Cat{}),
    ];
    
    for animal in animals {
        println!("{}", animal.name());
    }
}

fn get_animal(animal: AnimalType) -> Box::<dyn Animal>{
    match animal { 
        AnimalType::Dog => Box::new(Dog {}),
        AnimalType::Cat => Box::new(Cat {})
    }
}
