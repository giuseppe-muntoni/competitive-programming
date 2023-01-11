use holiday_planning::holiday_planning;
use std::io;
fn main() -> Result<(), std::io::Error> {
    let stdin = io::stdin();

    let mut input = String::new();
    let (n, d) = match stdin.read_line(&mut input) {
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

    let mut city_attractions = Vec::with_capacity(n);
    for i in 0..n {
        city_attractions.insert(
            i,
            match stdin.read_line(&mut input) {
                Ok(_) => {
                    let vals_str: Vec<&str> = input.split(' ').collect();
                    assert_eq!(d, vals_str.len());
                    vals_str
                        .iter()
                        .map(|x| x.trim().parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                }
                Err(e) => return Err(e),
            },
        );
        input.clear();
    }

    let result = holiday_planning(city_attractions);

    println!("{}", result);

    Ok(())
}
