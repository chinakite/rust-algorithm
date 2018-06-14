use std::cmp::Ordering;

pub fn quicksort<T>(arr: &mut [T]) where T: Ord {
    if arr.len() <= 1 {
        return
    }

    let len = arr.len();
    sort_helper(arr, 0, (len - 1) as isize, |a, b| a.cmp(b));
}

fn sort_helper<T, F>(arr: &mut [T], left: isize, right: isize, compare: F) where F: Fn(&T, &T) -> Ordering {
    sort(arr, left, right, &compare);
}

fn sort<T, F>(arr: &mut [T], left: isize, right: isize, compare: &F) where F: Fn(&T, &T) -> Ordering {
    if right <= left {
        return
    }

    let mut i:isize = left -1;
    let mut j:isize = right;

    let v: *mut T = &mut arr[right as usize];
    unsafe {
        loop {
            i += 1;
            while compare(&arr[i as usize], &*v) == Ordering::Less {
                i += 1
            }
            j -= 1;
            while compare(&*v, &arr[j as usize]) == Ordering::Less {
                if j == left {
                    break
                }
                j -= 1;
            }
            if i >= j {
                break
            }
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap(i as usize, right as usize);
    j = i - 1;
    i = i + 1;
    sort(arr, left, j, compare);
    sort(arr, i, right, compare);
}
