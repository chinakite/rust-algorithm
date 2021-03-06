mod sort;

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests {
    use rand::{self, Rng};
    use sort::quicksort;
    use sort::mergesort;
    use sort::heapsort;
    use sort::insertionsort;

    // #[test]
    fn quicksort() {
        const ARR_LEN:u32 = 10;

        let mut rng = rand::thread_rng();
        let mut arr = [0u32; ARR_LEN as usize];

        for i in 0..ARR_LEN as usize {
            let x: u32 = rng.gen::<u32>();
            arr[i] = x % ARR_LEN;
            println!("{}", arr[i]);
        }

        println!("======== quick sort ========");

        quicksort::quicksort(&mut arr);

        for i in 0..ARR_LEN as usize {
            println!("{}", arr[i]);
        }
        println!("======== quick sort end ========");
    }

    // #[test]
    fn mergesort() {
        const ARR_LEN:u32 = 10;

        let mut rng = rand::thread_rng();
        let mut arr = [0u32; ARR_LEN as usize];

        for i in 0..ARR_LEN as usize {
            let x: u32 = rng.gen::<u32>();
            arr[i] = x % ARR_LEN;
            println!("{}", arr[i]);
        }

        println!("======== merge sort ========");

        mergesort::mergesort(&mut arr);

        for i in 0..ARR_LEN as usize {
            println!("{}", arr[i]);
        }

        println!("======== merge sort end ========");
    }

    // #[test]
    fn heapsort() {
        const ARR_LEN:u32 = 10;

        let mut rng = rand::thread_rng();
        let mut arr = [0u32; ARR_LEN as usize];

        for i in 0..ARR_LEN as usize {
            let x: u32 = rng.gen::<u32>();
            arr[i] = x % ARR_LEN;
            println!("{}", arr[i]);
        }

        println!("======== heap sort ========");

        heapsort::heapsort(&mut arr);

        for i in 0..ARR_LEN as usize {
            println!("{}", arr[i]);
        }

        println!("======== heap sort end ========");
    }

    #[test]
    fn insertionsort() {
        const ARR_LEN:u32 = 10;
        let mut rng = rand::thread_rng();
        let mut arr = [0u32; ARR_LEN as usize];

        for i in 0..ARR_LEN as usize {
            let x: u32 = rng.gen::<u32>();
            arr[i] = x % ARR_LEN;
            println!("{}", arr[i]);
        }

        println!("======== insertion sort ========");
        insertionsort::insertionsort(&mut arr);

        for i in 0..ARR_LEN as usize {
            println!("{}", arr[i]);
        }

        println!("======== insertion sort end ========");
    }
}
