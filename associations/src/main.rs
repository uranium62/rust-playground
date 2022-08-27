use std::fmt::Debug;

trait Producer {
    type Input: Debug + Default;
    type Output: Debug + Default;
    
    fn produce(&self, input: Self::Input) -> Self::Output;
}

trait Generic<I: Debug + Default, O: Debug + Default> {
    fn produce(&self, input: I) -> O;
}

fn use_producer1(p: impl Producer) {
    
}
fn use_producer2(p: impl Producer<Input=u32, Output=String>) {

}
fn use_generic1<I: Debug + Default, O: Debug + Default>(g: impl Generic<I, O>) {
    
}
fn use_generic2<I, O>(g: impl Generic<I, O>)
    where I: Debug + Default, 
          O: Debug + Default
{
}

struct A {}

impl Producer for A {
    type Input = String;
    type Output = String;

    fn produce(&self, input: Self::Input) -> Self::Output {
        String::new()
    }
}

impl<I, O> Generic<I, O> for A
    where I: Debug + Default,
          O: Debug + Default
{
    fn produce(&self, input: I) -> O {
        O::default()
    }
}

fn main() {
    let a = A {};
    Producer::produce(&a, String::new());
    println!("Hello, world!");
}
