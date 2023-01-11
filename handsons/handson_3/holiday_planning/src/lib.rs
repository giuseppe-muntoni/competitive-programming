#[derive(Clone)]
struct NAttractionCities {
    n: i64,
    cities: Vec<bool>,
}

pub fn holiday_planning(city_attractions: Vec<Vec<i64>>) -> i64 {
    let ndays = city_attractions[0].len();
    let ncities = city_attractions.len();
    let mut max_attractions = vec![
        NAttractionCities {
            n: 0,
            cities: vec![false; ncities],
        };
        ndays + 1
    ];

    for n_attraction_city in max_attractions.iter_mut().skip(1).take(ndays) {
        let mut cities = vec![false; ncities];
        cities[0] = true;
        n_attraction_city.n = city_attractions[0][0];
        n_attraction_city.cities = cities;
    }

    let mut v = vec![0; ncities];
    v[0] = city_attractions[0][0];
    for w in 1..ndays + 1 {
        for (c, _) in city_attractions.iter().enumerate().take(ncities) {
            if w == 1 && c == 0 {
                continue;
            }

            v[c] += city_attractions[c][w - 1];

            for j in (1..ndays + 1).rev() {
                if (j as i32 - w as i32) < 0 {
                    continue;
                }

                if max_attractions[j - w].cities[c] {
                    continue;
                }

                if max_attractions[j - w].n + v[c] < max_attractions[j].n {
                    continue;
                }

                max_attractions[j].n = max_attractions[j - w].n + v[c];
                max_attractions[j].cities = max_attractions[j - w].cities.clone();
                max_attractions[j].cities[c] = true;
            }
        }
    }

    max_attractions[ndays].n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let florence = vec![3, 2, 1];
        let london = vec![3, 1, 1];
        let city_attractions = vec![florence, london];

        assert_eq!(8, holiday_planning(city_attractions))
    }

    #[test]
    fn test_2() {
        let a = vec![1, 1, 1, 4, 2];
        let b = vec![3, 3, 5, 3, 5];
        let c = vec![2, 1, 4, 5, 1];
        let city_attractions = vec![a, b, c];

        assert_eq!(19, holiday_planning(city_attractions))
    }

    #[test]
    fn test_3() {
        let a = vec![4, 1, 1, 2];
        let b = vec![1, 1, 0, 5];
        let c = vec![5, 0, 1, 1];
        let d = vec![2, 1, 0, 4];
        let e = vec![3, 1, 0, 3];
        let city_attractions = vec![a, b, c, d, e];

        assert_eq!(14, holiday_planning(city_attractions))
    }

    #[test]
    fn test_4() {
        let a = vec![1, 0, 2, 2, 0, 0, 2, 2, 0, 0, 0, 2, 0, 2, 2];
        let b = vec![0, 1, 1, 0, 0, 1, 2, 2, 1, 1, 1, 1, 2, 2, 1];
        let city_attractions = vec![a, b];

        assert_eq!(16, holiday_planning(city_attractions))
    }
}
