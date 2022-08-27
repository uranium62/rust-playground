use std::cell::RefCell;

struct Dev {
    name: String,
    lang: RefCell<String>
}

impl Dev {
    fn new<S: Into<String>>(name: S, lang: S) -> Self {
        Self {
            name: name.into(),
            lang: RefCell::new(lang.into())
        }
    }
    
    fn assign<S: Into<String>>(&self, lang: S){
        *self.lang.borrow_mut() = lang.into();
    }
}

fn main() {
    let dev = Dev::new("Bob Miller", "java");
    
    println!("{} - {}", dev.name, dev.lang.borrow());
    dev.assign("rust");
    println!("{} - {}", dev.name, dev.lang.borrow());
    dev.assign("dart");
    println!("{} - {}", dev.name, dev.lang.borrow());
}
