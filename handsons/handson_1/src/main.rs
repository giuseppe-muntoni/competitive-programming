use std::env;
use std::fs;
use std::time::Instant;

use sliding_window_maximum::{
    brute_force, brute_force_idiomatic, bst, gen_random_vector, heap, linear,
};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let ns = [
        //        1024,
        //        2 * 1024,
        //        4 * 1024,
        //        8 * 1024,
        //        16 * 1024,
        //        32 * 1024,
        64 * 1024,
        //        128 * 1024,
    ];
    let ks = [
        4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32000, 48000, 60000,
    ];

    // Write csv header
    let mut output_text: String = "Method,n,k,elapsed\n".to_string();

    for &n in ns.iter() {
        for &k in ks.iter() {
            if k as usize > n {
                continue;
            }
            let nums = gen_random_vector(n);

            // Brute force
            let (elapsed_times, _) = measure_elapsed_time(brute_force, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "BruteForce", n, k, min_elapsed);
            output_text.push_str(&row);

            // Brute force idiomatic
            let (elapsed_times, _) = measure_elapsed_time(brute_force_idiomatic, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "BruteForceIdiomatic", n, k, min_elapsed);
            output_text.push_str(&row);

            // Heap
            let (elapsed_times, _) = measure_elapsed_time(heap, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "Heap", n, k, min_elapsed);
            output_text.push_str(&row);

            // Bst
            let (elapsed_times, _) = measure_elapsed_time(bst, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "BST", n, k, min_elapsed);
            output_text.push_str(&row);

            // Linear
            let (elapsed_times, _) = measure_elapsed_time(linear, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "Linear", n, k, min_elapsed);
            output_text.push_str(&row);
        }
    }

    for &n in ns.iter() {
        for &k in ks.iter() {
            if k as usize > n {
                continue;
            }

            //Best case Heap
            let nums = generate_increasing_vector(n as i32);
            let (elapsed_times, _) = measure_elapsed_time(heap, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "HeapBest", n, k, min_elapsed);
            output_text.push_str(&row);

            //Worst case Heap
            let nums = generate_decreasing_vector(n as i32);
            let (elapsed_times, _) = measure_elapsed_time(heap, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "HeapWorst", n, k, min_elapsed);
            output_text.push_str(&row);

            //Worst case Linear
            let nums = generate_increasing_vector(n as i32);
            let (elapsed_times, _) = measure_elapsed_time(linear, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "LinearWorst", n, k, min_elapsed);
            output_text.push_str(&row);

            //Worst case BST
            let nums = generate_decreasing_vector(n as i32);
            let (elapsed_times, _) = measure_elapsed_time(bst, &nums, k);
            let min_elapsed = *elapsed_times.iter().min().unwrap();
            let row = format!("{},{},{},{}\n", "BSTWorst", n, k, min_elapsed);
            output_text.push_str(&row);
        }
    }

    //Best case Linear
    for &n in ns.iter() {
        let k = (n as f32 * 0.9) as i32;
        let nums = generate_decreasing_vector(n as i32);
        let (elapsed_times, _) = measure_elapsed_time(linear, &nums, k);
        let min_elapsed = *elapsed_times.iter().min().unwrap();
        let row = format!("{},{},{},{}\n", "LinearBest", n, k, min_elapsed);
        output_text.push_str(&row);
    }

    for &n in ns.iter() {
        let nums = gen_random_vector(n);

        //Best case Brute Force
        let (elapsed_times, _) = measure_elapsed_time(brute_force, &nums, 1);
        let min_elapsed = *elapsed_times.iter().min().unwrap();
        let row = format!("{},{},{},{}\n", "BruteForceBest", n, 1, min_elapsed);
        output_text.push_str(&row);

        //Best case Brute Force Idiomatic
        let (elapsed_times, _) = measure_elapsed_time(brute_force_idiomatic, &nums, 1);
        let min_elapsed = *elapsed_times.iter().min().unwrap();
        let row = format!(
            "{},{},{},{}\n",
            "BruteForceIdiomaticBest", n, 1, min_elapsed
        );
        output_text.push_str(&row);

        //Best case BST
        let (elapsed_times, _) = measure_elapsed_time(bst, &nums, 1);
        let min_elapsed = *elapsed_times.iter().min().unwrap();
        let row = format!("{},{},{},{}\n", "BSTBest", n, 1, min_elapsed);
        output_text.push_str(&row);

        //Worst case Brute Force
        let (elapsed_times, _) = measure_elapsed_time(brute_force, &nums, ((n as i32) / 2) as i32);
        let min_elapsed = *elapsed_times.iter().min().unwrap();
        let row = format!(
            "{},{},{},{}\n",
            "BruteForceWorst",
            n,
            ((n as i32) / 2) as i32,
            min_elapsed
        );
        output_text.push_str(&row);

        //Worst case Brute Force Idiomatic
        let (elapsed_times, _) =
            measure_elapsed_time(brute_force_idiomatic, &nums, ((n as i32) / 2) as i32);
        let min_elapsed = *elapsed_times.iter().min().unwrap();
        let row = format!(
            "{},{},{},{}\n",
            "BruteForceIdiomaticWorst",
            n,
            ((n as i32) / 2) as i32,
            min_elapsed
        );
        output_text.push_str(&row);
    }

    let output_path = "results_native.csv";
    fs::write(output_path, output_text).expect("Unable to write file");
}

const N_RUNS: usize = 5;

fn measure_elapsed_time(
    f: fn(vec: &Vec<i32>, k: i32) -> Vec<i32>,
    nums: &Vec<i32>,
    k: i32,
) -> (Vec<u128>, Vec<i32>) {
    let mut elapsed_times: Vec<u128> = Vec::with_capacity(N_RUNS);
    let mut results = Vec::new();
    for _ in 0..N_RUNS {
        let start = Instant::now();
        results = f(nums, k);
        let duration = start.elapsed().as_nanos();
        elapsed_times.push(duration);
    }

    (elapsed_times, results)
}

fn generate_increasing_vector(n: i32) -> Vec<i32> {
    (0..n).collect()
}

fn generate_decreasing_vector(n: i32) -> Vec<i32> {
    (0..n).rev().collect()
}
