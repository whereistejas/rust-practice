use std::ops::Mul;

pub fn multiply<T, U>(a: Vec<T>, b: Vec<U>) -> Vec<U>
where
    T: Mul<U, Output = U>
{
    a.into_iter()
        .zip(b.into_iter())
        .map(|(val1, val2)| val1 * val2)
        .collect()
}
fn main() {
    let a: Vec<i8> = vec![1, 2, 3, 4, 5];
    let b: Vec<i32> = vec![1, 4, 2, 1, 0];

    let result = multiply(a.iter().map(|&element| element as i32).collect(), b);
    println!("{:?}", result);
}
