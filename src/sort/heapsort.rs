use std::cmp::Ordering;

pub fn heapsort<T>(arr: &mut [T]) where T: Ord {
    let len = arr.len();
    if len <= 1 {
        return
    }

    build_maxheap(arr);

    let mut i = len - 1;
    while i > 0 {
        arr.swap(0, i);
        max_heapify(arr, 0, i, |a, b| a.cmp(b));
        i = i - 1;
    }
}

fn build_maxheap<T>(arr: &mut [T]) where T: Ord {
    let len: usize = arr.len();
    let mut parent = (len / 2) as usize;
    parent = parent - 1;

    let mut i = parent as isize;
    while i >= 0 {
        max_heapify(arr, i as usize, len, |a, b| a.cmp(b));
        i = i-1;
    }
}

fn max_heapify<T, F>(arr: &mut [T], mut index: usize, heapsize: usize, compare: F) where T: Ord, F: Fn(&T, &T) -> Ordering {
    let mut max = index;
    let mut left = 2*index + 1;
    let mut right = 2*(index+1);

    if left < heapsize && compare(&arr[index], &arr[left]) == Ordering::Less {
        max = left;
    }

    if right < heapsize && compare(&arr[max], &arr[right]) == Ordering::Less {
        max = right;
    }

    if max != index {
        arr.swap(max, index);
        max_heapify(arr, max, heapsize, compare);
    }

}
