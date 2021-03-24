use std::fmt::Debug;
use std::ops::Deref;

struct MyBox<T>(T);

struct CustomSmartPointer<T: Debug> {
    data: T,
}

impl<T> Drop for CustomSmartPointer<T>
where
    T: Debug,
{
    fn drop(&mut self) {
        println!("dropping csp with data: {:?}", self.data);
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let name = MyBox::new(String::from("name"));
    hello(&name);

    let csp1 = CustomSmartPointer {
        data: String::from("hello from hell"),
    };
    println!("{}", csp1.data);
}
