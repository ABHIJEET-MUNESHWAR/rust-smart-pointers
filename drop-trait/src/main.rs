struct CustomSmartPointer{
    data:String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}` !!!", self.data);
    }
}

fn main() {
    let _c = CustomSmartPointer{
        data: String::from("My Stuff"),
    };
    let _d = CustomSmartPointer{
        data: String::from("Other Stuff"),
    };
    println!("Custom smart pointers created.");
}
