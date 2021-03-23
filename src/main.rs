enum List<T>{
    Cons(T, Box<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list_int = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil) ) ) ) ) );
    let list_char = Cons('a', Box::new(Cons('b', Box::new(Cons('c', Box::new(Nil))))));
}
