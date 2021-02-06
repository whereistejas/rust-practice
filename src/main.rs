fn main() {
    // the goal here is to write a method that finds the minimum in a vector
    // of arbitrary type.
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let vec3 = vec!['a', 'b', 'c', 'd', 'e'];
    let vec4 = vec!["oranges", "oears", "oeaches", "oemons", "oangos"];

    println!("vector: {:?}, minimum: {}", vec1, find_min(&vec1));
    println!("vector: {:?}, minimum: {}", vec2, find_min(&vec2));
    println!("vector: {:?}, minimum: {}", vec3, find_min(&vec3));
    println!("vector: {:?}, minimum: {}", vec4, find_min(&vec4));
}

fn find_min<T: PartialOrd> (vector: &Vec<T>) -> &T {
    let mut min = vector.get(0).unwrap();
    for item in vector.iter() {
        if *item < *min {
            min = item;
        }
    }
    min
}
