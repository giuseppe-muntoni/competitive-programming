use binary_search_tree::BinarySearchTree;
use rand::Rng;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

pub fn brute_force(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = v.len();
    let mut maximums = Vec::with_capacity(n - k + 1);
    for i in 0..(n - k + 1) {
        let current_slice = &v[i..i + k];
        let max_value = *current_slice.iter().max().unwrap();
        maximums.push(max_value);
    }
    maximums
}

pub fn brute_force_idiomatic(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    v.windows(k).map(|w| *w.iter().max().unwrap()).collect()
}

pub fn heap(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    if k > n || k == 0 {
        Vec::with_capacity(0)
    } else {
        let mut heap = BinaryHeap::new();
        let mut maximums = Vec::with_capacity(n - k + 1);
        for i in 0..n {
            heap.push((nums.get(i).unwrap(), i));
            if i < k {
                if i == k - 1 {
                    maximums.push(*(heap.peek().unwrap().0));
                }
            } else {
                let mut found = false;
                while !found {
                    let (element, index) = *(heap.peek().unwrap());
                    if index < i - k + 1 {
                        heap.pop();
                    } else {
                        maximums.push(*element);
                        found = true;
                    }
                }
            }
        }
        maximums
    }
}

pub fn bst(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    if k > n || k == 0 {
        Vec::with_capacity(0)
    } else {
        let mut tree = BinarySearchTree::new();
        let mut maximums = Vec::with_capacity(n - k + 1);
        for i in 0..n {
            tree.insert((*nums.get(i).unwrap(), i));
            if i < k {
                if i == k - 1 {
                    maximums.push(tree.max().unwrap().0);
                }
            } else {
                tree.remove(&(*nums.get(i - k).unwrap(), i - k));
                maximums.push(tree.max().unwrap().0);
            }
        }
        maximums
    }
}

pub fn linear(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    if k as usize > n || k == 0 {
        Vec::with_capacity(0)
    } else {
        let mut deque: VecDeque<(i32, i32)> = VecDeque::with_capacity(k as usize);
        let mut maximums = Vec::with_capacity(n - k as usize + 1);
        for i in 0..n {
            let element = *nums.get(i).unwrap();
            while !deque.is_empty() && deque.back().unwrap().0 <= element {
                deque.pop_back();
            }
            deque.push_back((element, i as i32));
            while deque.front().unwrap().1 <= (i as i32 - k) {
                deque.pop_front();
            }
            if i as i32 >= k - 1 {
                maximums.push(deque.front().unwrap().0);
            }
        }
        maximums
    }
}

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        nums.push(rng.gen_range(0..i32::MAX));
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idiomatic_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = brute_force_idiomatic(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }
}
