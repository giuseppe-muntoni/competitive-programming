use std::io;
use xmas_lights::xmas_lights;

fn main() -> Result<(), std::io::Error> {
    let stdin = io::stdin();

    let mut input = String::new();

    let n = match stdin.read_line(&mut input) {
        Ok(_) => input.trim().parse::<usize>().unwrap(),
        Err(e) => return Err(e),
    };

    input.clear();

    let h: Vec<char> = match stdin.read_line(&mut input) {
        Ok(_) => input.trim().chars().collect(),
        Err(e) => return Err(e),
    };

    input.clear();

    assert_eq!(n, h.len());

    let result = xmas_lights(h);

    println!("{}", result);

    Ok(())
}
