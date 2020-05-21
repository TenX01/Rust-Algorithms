

mod algorithms;



#[cfg(test)]
mod tests {
    use super::algorithms::sort_methods::*;


    #[test]
    fn test_insertion_sort() {
        use rand::Rng;
        use rand::prelude::*;
        let mut nums_set: Vec<Vec<i32>> = vec![];
        let mut rng = rand::thread_rng();
        let times = rng.gen_range(10, 20);
        for _i in 0..times {
            let upper = rng.gen_range(10, 20);
            let mut nums: Vec<i32> = (1..upper).collect();
            nums.shuffle(&mut rng);
            nums_set.push(nums);
        }

        for num in &mut nums_set {
            let mut num_copy = num.clone();
            num_copy.sort();
            insertion_sort(num);
            assert_eq!(num_copy, *num);
        }
    }

}