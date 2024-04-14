use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the number of elements you want in the array");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Not a number");
    let mut nums = Vec::<i32>::new();

    println!("Enter elements of the array");
    let mut count = 0;
    while count < n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Not a number");
        nums.push(num);
        count += 1;
    }

    nums.sort();
    input.clear();
    println!("Enter the number which you want to find in the array");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target: i32 = input.trim().parse().expect("Not a number");

    match find_first_occurrence(&nums, target) {
        Some(index) => println!("{} is present at index {}.", target, index),
        None => println!("{} is not present in the array.", target),
    }
}

fn find_first_occurrence(nums: &[i32], target: i32) -> Option<i32> {
    let (mut left, mut right) = (0, nums.len());

    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left < nums.len() && nums[left] == target {
        Some(left as i32)
    } else {
       None
    }
}
