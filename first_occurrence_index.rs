fn first_occurrence_index(nums: &[i32], target: i32) -> Option<usize> {
    for (index, &num) in nums.iter().enumerate() {
        if num == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 5, 6, 7, 8];
    let target = 2;
    if let Some(index) = first_occurrence_index(&nums, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}
