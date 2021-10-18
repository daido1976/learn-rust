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

#[test]
fn test_quick_sort_in_place() {
    let mut arr = [-5, 4, 1, -3, 2];
    quick_sort_in_place(&mut arr);
    assert_eq!(arr, [-5, -3, 1, 2, 4]);

    let mut arr = ['b', 'a', 'c'];
    quick_sort_in_place(&mut arr);
    assert_eq!(arr, ['a', 'b', 'c']);
}

// See. https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/quick_sort.rs
fn quick_sort_in_place<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);

    fn _quick_sort<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
        if lo < hi {
            let p = _partition(arr, lo, hi);
            _quick_sort(arr, lo, p - 1);
            _quick_sort(arr, p + 1, hi);
        }
    }

    fn _partition<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> isize {
        let pivot = hi as usize;
        let mut i = lo - 1;
        let mut j = hi;

        loop {
            i += 1;
            while arr[i as usize] < arr[pivot] {
                i += 1;
            }
            j -= 1;
            while j >= 0 && arr[j as usize] > arr[pivot] {
                j -= 1;
            }
            if i >= j {
                break;
            } else {
                arr.swap(i as usize, j as usize);
            }
        }
        arr.swap(i as usize, pivot as usize);
        i
    }
}
