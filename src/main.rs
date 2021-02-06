fn main() {
    let mut vector = vec![1, 2, 3, 4, 5];
    let element0 = vector.get(0);
    let element3 = vector.get_mut(2);
    *element3.unwrap() = 42;
    println!("element0: {}", element0);
    println!("vector: {:?}", vector);
}
