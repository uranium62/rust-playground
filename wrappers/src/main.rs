fn main() {
    type_box();
}

fn type_box(){
    let x = Box::new(1);
    let y = x;

    println!("{:?}", y);
}

