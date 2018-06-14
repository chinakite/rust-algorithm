mod sort;

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests {
    use rand::{self, Rng};
    use sort::quicksort;

    #[test]
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
    }
}
