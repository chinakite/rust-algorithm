use std::cmp::Ordering;
use std::fmt::Display;

pub fn insertionsort<T>(arr: &mut [T]) where T: Ord + Copy + Display {
    let len = arr.len();
    if len <= 1 {
        return
    }

    sort(arr, |a, b| a.cmp(b));
}

fn sort<T, F>(arr: &mut [T], compare: F) where T: Ord + Copy + Display, F: Fn(&T, &T) -> Ordering {
    let len = arr.len();
    let mut i = 1;
    let mut j = 0;
    let mut k: usize = 0;
    let mut temp: T;

    while i < len {
        temp = arr[i];

        if compare(&arr[i-1], &temp) == Ordering::Less
            || compare(&arr[i-1], &temp) == Ordering::Equal
        {
            k = i;
        }else{
            k = binary_search(arr, 0, i-1, &temp, &compare);
            j = i;
            while j > k {
                arr[j] = arr[j-1];
                j = j-1;
            }
        }
        arr[k] = temp;
        i = i + 1;
    }
}

fn binary_search<T, F>(arr: &mut [T], mut start: usize, mut end: usize, temp: &T, compare: &F) -> usize where T: Ord + Copy, F: Fn(&T, &T) -> Ordering{
    while start <= end {
        let middle: usize = (start + end)/2 as usize;
        if compare(&arr[middle], temp) == Ordering::Less
            || compare(&arr[middle], temp) == Ordering::Equal
        {
            if compare(temp, &arr[middle + 1]) == Ordering::Less {
                return middle + 1;
            }else{
                start = middle + 1;
            }
        }else{
            if end == 0 {
                return 0;
            }else{
                end = middle;
            }
        }
    }
    return 0;
}
