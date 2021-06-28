pub fn bubble_sort(mut list: Vec<i32>) -> Vec<i32> {

    loop {
        let mut swapped = false;

        for i in 1..list.len() {
            if list[i-1] > list[i] {
                list.swap(i - 1, i);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bubble_sort() {
        assert_eq!(vec![1, 2, 3, 4, 5], bubble_sort(vec![4, 3, 2, 1, 5]));
    }
}
