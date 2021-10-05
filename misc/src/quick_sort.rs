#[test]
fn test_quick_sort() {
    assert_eq!(quick_sort(vec![-5, 4, 1, -3, 2]), vec![-5, -3, 1, 2, 4]);
    assert_eq!(quick_sort(vec![1]), vec![1]);
    assert_eq!(quick_sort(vec![]), vec![]);
}

// Language: haskell
// quickSort :: (Ord a) => [a] -> [a]
// quickSort [] = []
// quickSort (x : xs) =
//   let smallerOrEqual = [a | a <- xs, a <= x]
//       bigger = [a | a <- xs, a > x]
//    in quickSort smallerOrEqual ++ [x] ++ quickSort bigger
fn quick_sort(mut list: Vec<i32>) -> Vec<i32> {
    if list.len() <= 1 {
        return list;
    }

    let pivot = list.pop().unwrap();
    let mut left = Vec::new();
    let mut right = Vec::new();
    list.iter().for_each(|x| {
        if x <= &pivot {
            // smallerOrEqual
            left.push(*x);
        } else {
            // bigger
            right.push(*x);
        }
    });

    let mut result = quick_sort(left);
    result.push(pivot);
    result.append(&mut quick_sort(right));
    result
}
