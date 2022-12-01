use std::io;

use min_max::Query;
fn main() -> Result<(), std::io::Error> {
    let stdin = io::stdin();

    let mut input = String::new();
    let (n, m) = match stdin.read_line(&mut input) {
        Ok(_) => {
            let dims: Vec<&str> = input.split(' ').collect();
            assert_eq!(2, dims.len());
            (
                dims[0].trim().parse::<usize>().unwrap(),
                dims[1].trim().parse::<usize>().unwrap(),
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

    let mut queries: Vec<Query> = Vec::with_capacity(m);
    for _ in 0..m {
        input.clear();
        queries.push(match stdin.read_line(&mut input) {
            Ok(_) => {
                let vals_str: Vec<&str> = input.split(' ').collect();
                assert!(vals_str.len() > 1);
                let query_type = vals_str[0].trim().parse::<i8>().unwrap();
                assert!(query_type == 0 || query_type == 1);
                let v = if query_type == 0 {
                    assert_eq!(4, vals_str.len());
                    Some(vals_str[3].trim().parse::<i64>().unwrap())
                } else {
                    assert_eq!(3, vals_str.len());
                    None
                };
                Query {
                    l: vals_str[1].trim().parse::<usize>().unwrap(),
                    r: vals_str[2].trim().parse::<usize>().unwrap(),
                    v,
                }
            }
            Err(e) => return Err(e),
        });
    }

    let result = min_max::min_max(vals, queries);

    result.iter().for_each(|x| println!("{x}"));

    Ok(())
}
