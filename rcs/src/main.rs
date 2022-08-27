use std::rc::Rc;

#[derive(Debug)]
struct Dev {
    lang: String,
    name: String,
}

#[derive(Debug)]
struct Project {
    name: String,
    dev: Rc<Dev>,
}

fn main() {
    let dev = Rc::new(Dev { 
        lang: "rust".to_string(),
        name: "Jack Miller".to_string()
    });
    
    println!("rc counts: {}", Rc::strong_count(&dev));
    
    let dev1 = dev.clone();
    let dev2 = dev.clone();
    
    let project1 = Project {
        name: "google maps".to_string(),
        dev: dev1
    };
    let project2 = Project {
        name: "google search".to_string(),
        dev: dev2
    };

    println!("rc counts: {}", Rc::strong_count(&dev));
    println!("{:?}", project1);
    println!("{:?}", project2);
}
