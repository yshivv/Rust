fn find_median(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len == 0 {
        return 0.0; // Return 0.0 for empty array (edge case)
    }
    if len % 2 == 1 {
        // Odd number of elements
        nums[len / 2] as f64
    } else {
        // Even number of elements
        let mid = len / 2;
        (nums[mid - 1] as f64 + nums[mid] as f64) / 2.0
    }
}

fn main() {
    let nums = vec![1, 3, 5, 7, 9]; // Example sorted array
    let median = find_median(&nums);
    println!("Median of the array: {}", median);
}
