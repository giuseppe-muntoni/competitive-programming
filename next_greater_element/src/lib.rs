use std::collections::VecDeque;

pub fn next_greater_element(nums: Vec<i32>) -> Vec<i32> {
    let mut deque = VecDeque::with_capacity(nums.len());
    let mut result = vec![-1; nums.len()];

    let mut max_idx = 0;
    let mut max = i32::MIN;

    for i in 0..nums.len() {
        if nums[i] > max {
            max = nums[i];
            max_idx = i;
        }
    }
    
    deque.push_back(max);
    let mut i = if max_idx == 0 {
        nums.len() - 1
    } else {
        max_idx - 1
    };
    
    while i != max_idx {
        while !deque.is_empty() && *deque.back().unwrap() <= nums[i] {
            deque.pop_back();
        }
        result[i] = *deque.back().or(Some(&-1)).unwrap();
        deque.push_back(nums[i]);
        i = if i == 0 {
            nums.len() - 1
        } else {
            i - 1
        }
    }

    result
}