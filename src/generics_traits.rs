use std::ops::Mul;

pub fn add<T, U>(a: Vec<T>, b: Vec<U>) -> Vec<U>
where
    T: Mul<T, Output = U> + Mul<U, Output = U>
{
    a.into_iter()
        .zip(b.into_iter())
        .map(|(val1, val2)| val1 * val2)
        .collect()
}