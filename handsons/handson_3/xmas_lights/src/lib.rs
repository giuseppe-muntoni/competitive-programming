pub fn xmas_lights(h: Vec<char>) -> i64 {
    let mut patriotic_selections = 0;
    let num_houses = h.len();

    let mut w = vec![0; num_houses];
    let mut g = vec![0; num_houses];

    let mut power_of_3 = 1;

    if h[num_houses - 1] == 'X' {
        g[num_houses - 1] = 1;
        power_of_3 *= 3;
    } else if h[num_houses - 1] == 'G' {
        g[num_houses - 1] = 1;
    }

    for i in (0..num_houses - 1).rev() {
        if h[i] == 'G' {
            g[i] = g[i + 1] + power_of_3;
        } else if h[i] == 'X' {
            g[i] = 3 * g[i + 1] + power_of_3;
            power_of_3 *= 3;
        } else {
            g[i] = g[i + 1];
        }
    }

    for i in (0..num_houses - 1).rev() {
        if h[i] == 'W' {
            w[i] = w[i + 1] + g[i + 1];
        } else if h[i] == 'X' {
            w[i] = 3 * w[i + 1] + g[i + 1];
        } else {
            w[i] = w[i + 1];
        }
    }

    for i in (0..num_houses - 1).rev() {
        if h[i] == 'R' {
            patriotic_selections += w[i + 1];
        } else if h[i] == 'X' {
            patriotic_selections *= 3;
            patriotic_selections += w[i + 1];
        }
    }

    patriotic_selections
}

#[cfg(test)]
mod tests {
    use super::xmas_lights;

    #[test]
    fn test_1() {
        let h = vec!['X', 'X', 'X'];
        assert_eq!(xmas_lights(h), 1);
    }

    #[test]
    fn test_2() {
        let h = vec!['X', 'X', 'X', 'X', 'X', 'X', 'X'];
        assert_eq!(xmas_lights(h), 2835);
    }

    #[test]
    fn test_3() {
        let h = vec!['R', 'W', 'G', 'X', 'X'];
        assert_eq!(xmas_lights(h), 16);
    }

    #[test]
    fn test_4() {
        let h = vec!['R', 'W', 'X'];
        assert_eq!(xmas_lights(h), 1);
    }

    #[test]
    fn test_5() {
        let h = vec!['R', 'G', 'X', 'W', 'X', 'G'];
        assert_eq!(xmas_lights(h), 24);
    }

    #[test]
    fn test_6() {
        let h = vec!['G', 'G', 'G', 'R', 'R', 'R', 'W', 'W', 'W'];
        assert_eq!(xmas_lights(h), 0);
    }

    #[test]
    fn test_7() {
        let h = vec!['G', 'G', 'W', 'X', 'X', 'R', 'W'];
        assert_eq!(xmas_lights(h), 0);
    }
}
