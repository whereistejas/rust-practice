fn main() {
    let num_list = vec![1, 2, 3, 9, 8, 5];

    let largest = largest(&num_list);

    println!("largest: {}", largest);
}

// additions from The Rust Book
// use of references
//
// peculiar observation: if we return a pointer from a function and
// then assign it to a variable, it automatically takes the value to
// which the reference is reffering.

fn largest(num_list: &Vec<i64>) -> &i64 {
    let mut largest = &num_list[0];

    for num in num_list {
        if num > largest {
            largest = num;
        }
    }

    largest
}
