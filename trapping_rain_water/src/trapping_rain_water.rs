use std::collections::VecDeque;

pub fn trap(height: Vec<i32>) -> i32 {
    let mut leaders = VecDeque::with_capacity(height.len());
    let mut leader = i32::MIN;
    let mut water = 0;

    for i in (0..height.len()).rev() {
        if height[i] >= leader {
            leader = height[i];
            leaders.push_front(i);
        }
    }

    let mut wall = 0;
    for i in 0..height.len() - 1 {
        if i < wall || height[i] < height[wall] {
            water += height[wall] - height[i];
        } else if i >= wall && is_a_leader(i, &leaders) {
            leaders.pop_front();
            wall = *leaders.front().unwrap();
        } else {
            wall = i;
        }
    }
    water
}

fn is_a_leader(i: usize, leaders: &VecDeque<usize>) -> bool {
    match leaders.front() {
        Some(j) if *j == i => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::trap;

    #[test]
    fn test_1() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        assert_eq!(6, trap(height));
    }

    #[test]
    fn test_2() {
        let height = vec![4,2,0,3,2,5];
        assert_eq!(9, trap(height));
    }

    #[test]
    fn test_3() {
        let height = vec![4,2,3];
        assert_eq!(1, trap(height));
    }

}
