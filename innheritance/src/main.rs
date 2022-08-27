use crate::model::{Dev, DevOps, Tasks};

mod model {
    pub struct Dev {
        pub lang: String,
        tasks: Vec<String>,
    }
    impl Dev {
        pub fn new(lang: String, tasks: Vec<String>) -> Self {
            Self {
                lang,
                tasks,
            }
        }
    }
    
    pub struct DevOps {
        pub lang: String,
        tasks: Vec<String>,
    }
    impl DevOps {
        pub fn new(lang: String, tasks: Vec<String>) -> Self {
            Self {
                lang,
                tasks,
            }
        }
    }

    pub trait Tasks {
        fn get(&self) -> &Vec<String>;
    }
    impl Tasks for Dev {
        fn get(&self) -> &Vec<String> {
            &self.tasks
        }
    }
    impl Tasks for DevOps {
        fn get(&self) -> &Vec<String> {
            &self.tasks
        }
    }
}

fn main() {
    let dev1 = Dev::new("rust".to_string(), vec!["task1".to_string()]);
    let dev2 = Dev::new("java".to_string(), vec!["task2".to_string()]);

    let ops1 = DevOps::new("perl".to_string(), vec!["task1".to_string()]);
    let ops2 = DevOps::new("bash".to_string(), vec!["task2".to_string()]);

    println!("dev1 = {:?}", dev1.lang);
    println!("dev2 = {:?}", dev2.lang);

    println!("ops1 = {:?}", ops1.lang);
    println!("ops2 = {:?}", ops2.lang);
    
    let tasks: Vec<Box<dyn Tasks>> = vec![
        Box::new(dev1),
        Box::new(dev2),
        Box::new(ops1),
        Box::new(ops2),
    ];

    for task in tasks {
        println!("{:?}", task.get());
    }
}
