use std::cmp::Ordering;

pub fn mergesort<T>(arr: &mut [T]) where T: Ord + Copy {
    if arr.len() <= 1 {
        return
    }

    let len = arr.len();
    let mut tmp_arr:Vec<T> = Vec::with_capacity(len);
    unsafe {
        tmp_arr.set_len(len);
    }

    sort_helper(arr, 0, (len-1), &mut tmp_arr, |a,b| a.cmp(b));
}

fn sort_helper<T, F>(arr: &mut [T], first: usize, last: usize, tmp_arr: &mut Vec<T>, compare: F) where T: Ord + Copy, F: Fn(&T, &T) -> Ordering {
    sort(arr, first, last, tmp_arr, &compare);
}

fn sort<T, F>(arr: &mut [T], first: usize, last: usize, tmp_arr: &mut Vec<T>, compare: &F) where T: Ord + Copy, F: Fn(&T, &T) -> Ordering {
    if last <= first {
        return
    }

    let middle = (first + last) / 2;
    sort(arr, first, middle, tmp_arr, compare);
    sort(arr, middle+1, last, tmp_arr, compare);
    merge_array(arr, first, middle, last, tmp_arr, compare);
}

fn merge_array<T, F>(arr: &mut [T], first: usize, middle: usize, last: usize, tmp_arr: &mut Vec<T>, compare: &F) where T: Ord + Copy, F: Fn(&T, &T) -> Ordering {
    let mut i = first;
    let j = middle;
    let mut k = middle + 1;
    let m = last;
    let mut c = first;

    while i <= j && k <= m {
        if compare(&arr[i as usize], &arr[k as usize]) == Ordering::Less {
            tmp_arr[c] = arr[i as usize];
            c += 1;
            i += 1;
        } else {
            tmp_arr[c] = arr[k as usize];
            c += 1;
            k += 1;
        }
    }

    while i <= middle {
        tmp_arr[c] = arr[i as usize];
        c += 1;
        i += 1;
    }

    while k <= m {
        tmp_arr[c] = arr[k as usize];
        c += 1;
        k += 1;
    }

    let mut n = 0;
    while n <= m {
        arr[n as usize] = tmp_arr[n as usize];
        n += 1;
    }
}
