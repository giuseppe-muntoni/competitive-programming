use std::io;

use queries_and_ops::{Operation, Query};

fn main() -> Result<(), std::io::Error> {
    let stdin = io::stdin();

    let mut input = String::new();
    let (n, m, k) = match stdin.read_line(&mut input) {
        Ok(_) => {
            let dims: Vec<&str> = input.split(' ').collect();
            assert_eq!(3, dims.len());
            (
                dims[0].trim().parse::<usize>().unwrap(),
                dims[1].trim().parse::<usize>().unwrap(),
                dims[2].trim().parse::<usize>().unwrap(),
            )
        }
        Err(e) => return Err(e),
    };

    input.clear();
    let vals = match stdin.read_line(&mut input) {
        Ok(_) => {
            let vals_str: Vec<&str> = input.split(' ').collect();
            assert_eq!(n, vals_str.len());
            vals_str
                .iter()
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        }
        Err(e) => return Err(e),
    };

    let mut ops: Vec<Operation> = Vec::with_capacity(m);
    for _ in 0..m {
        input.clear();
        ops.push(match stdin.read_line(&mut input) {
            Ok(_) => {
                let vals_str: Vec<&str> = input.split(' ').collect();
                assert_eq!(3, vals_str.len());
                Operation {
                    l: vals_str[0].trim().parse::<usize>().unwrap(),
                    r: vals_str[1].trim().parse::<usize>().unwrap(),
                    v: vals_str[2].trim().parse::<i64>().unwrap(),
                }
            }
            Err(e) => return Err(e),
        });
    }

    let mut queries: Vec<Query> = Vec::with_capacity(k);
    for _ in 0..k {
        input.clear();
        queries.push(match stdin.read_line(&mut input) {
            Ok(_) => {
                let vals_str: Vec<&str> = input.split(' ').collect();
                assert_eq!(2, vals_str.len());
                Query {
                    l: vals_str[0].trim().parse::<usize>().unwrap(),
                    r: vals_str[1].trim().parse::<usize>().unwrap(),
                }
            }
            Err(e) => return Err(e),
        });
    }

    let result = queries_and_ops::queries_and_ops(vals, ops, queries);

    let mut output = String::new();
    for (i, item) in result.iter().enumerate().take(n) {
        output.push_str(&item.to_string());
        if i < n - 1 {
            output.push(' ');
        }
    }
    println!("{output}");

    Ok(())
}
