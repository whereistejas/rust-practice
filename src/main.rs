fn main() {
    let num_list = vec![1, 2, 3, 9, 8, 5];

    let largest = largest(&num_list);

    println!("largest: {}", largest);
}

// we are adding the Copy trait here, because the generic
// can be anything, so we have to move a heap datatype

fn largest<T: PartialOrd + Copy>(num_list: &Vec<T>) -> T {
    let mut largest = num_list[0];

    // here we are using `&num`, it doesn't mean ref of a ref; we just 
    // directly store the ref of the item from the list. so, later in
    // the for loop we can just use `num` as if it was derefing `&num`

    for &num in num_list {
        if num > largest {
            largest = num;
        }
    }

    largest
}
