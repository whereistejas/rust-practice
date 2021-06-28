/// reference: https://www.khanacademy.org/computing/computer-science/algorithms/merge-sort/a/overview-of-merge-sort
/// reference: https://github.com/pedrovgs/Algorithms/blob/master/src/main/java/com/github/pedrovgs/problem79/MergeSort.java

pub fn merge_sort(mut list: Vec<i32>) -> Vec<i32> {
    // check if we have a base case
    if list.len() <= 2 {
        // compare and if needed, swap
        if list.len() != 1 {
            if list[0] > list[1] {
                list.reverse();
            }
        }
    } else {
        // get the list length
        let len = list.len();

        // cut the list in two halves
        let first_half = merge_sort((&list[0..(len / 2)]).to_vec());
        let second_half = merge_sort((&list[(len / 2)..]).to_vec());

        list = merge(first_half, second_half)
    }
    dbg!(&list);

    list
}

fn merge(first_half: Vec<i32>, second_half: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    // implementation using iterators
    let mut iter1 = first_half.into_iter().peekable();
    let mut iter2 = second_half.into_iter().peekable();

    loop {
        match (iter1.peek(), iter2.peek()) {
            (Some(x), Some(y)) => {
                if *x < *y {
                    result.push(iter1.next().unwrap());
                } else {
                    result.push(iter2.next().unwrap());
                }
            }
            (.., None) => {
                result.extend(iter1);
                break;
            }
            (None, ..) => {
                result.extend(iter2);
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_even_list() {
        assert_eq!(vec![1, 2, 3, 4], merge_sort(vec![4, 1, 3, 2]));
    }

    #[test]
    fn check_odd_list() {
        assert_eq!(vec![1, 2, 3, 4, 5], merge_sort(vec![4, 1, 3, 2, 5]));
    }

    #[test]
    fn check_long_lists() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            merge_sort(vec![10, 9, 8, 3, 2, 1, 4, 5, 6, 7])
        );
    }

    #[test]
    fn check_duplicate_elements() {
        assert_eq!(
            vec![1, 2, 2, 3, 4, 4, 5],
            merge_sort(vec![2, 2, 4, 4, 1, 3, 5])
        );
    }
}
