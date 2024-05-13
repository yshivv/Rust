fn kth_smallest_element(nums: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= nums.len() {
        let mut sorted_nums = nums.to_vec();
        sorted_nums.sort();
        Some(sorted_nums[k - 1])
    } else {
        None
    }
}

fn main() {
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6, 5]; // Example array
    let k = 3; // Example value of k

    if let Some(kth_smallest) = kth_smallest_element(&nums, k) {
        println!("{}th smallest element in the array: {}", k, kth_smallest);
    } else {
        println!("Invalid value of k");
    }
}
