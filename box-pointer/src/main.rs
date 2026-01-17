enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use List::{Cons, Nil};

trait Vehicle{
    fn drive(&self);
}

struct Truck;

impl Vehicle for Truck{
    fn drive(&self){
        println!("Truck is driving");
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let t : Box<dyn Vehicle>;
    t = Box::new(Truck);
    t.drive();
}
