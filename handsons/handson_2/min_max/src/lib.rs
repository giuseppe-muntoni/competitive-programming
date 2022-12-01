use segment_tree::{self, SegmentTree, SegmentTreeSpec};

pub struct Query {
    pub l: usize,
    pub r: usize,
    pub v: Option<i64>,
}

pub enum MinMaxSpec {}
impl SegmentTreeSpec for MinMaxSpec {
    type S = i64;
    type F = i64;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a.max(b)
    }
    fn identity() -> Self::S {
        i64::min_value()
    }
    fn compose(&f: &Self::F, g: &Self::F) -> Self::F {
        f.min(*g)
    }
    fn apply(&f: &Self::F, a: &Self::S, _: i64) -> Self::S {
        f.min(*a)
    }
}

pub fn min_max(val: Vec<i64>, queries: Vec<Query>) -> Vec<i64> {
    let mut s_tree: SegmentTree<MinMaxSpec> = SegmentTree::new(val);
    let mut maximums: Vec<i64> = Vec::with_capacity(queries.len());

    for query_i in &queries {
        let query = query_i;
        match query.v {
            Some(t) => s_tree.update(query.l - 1, query.r, &t),
            None => maximums.push(s_tree.query(query.l - 1, query.r)),
        }
    }

    maximums
}

#[cfg(test)]
mod tests {
    use super::min_max;
    use super::Query;

    #[test]
    fn test_min_max_1() {
        let val = vec![5, 1, 4, 3, 2];
        let queries = vec![
            Query {
                l: 1,
                r: 2,
                v: Some(2),
            },
            Query {
                l: 2,
                r: 4,
                v: None,
            },
            Query {
                l: 1,
                r: 2,
                v: None,
            },
        ];

        let maximums = min_max(val, queries);

        assert_eq!(2, maximums.len());
        assert_eq!(4, maximums[0]);
        assert_eq!(2, maximums[1]);
    }

    #[test]
    fn test_min_max_2() {
        let val = vec![1, 4, 2, 3, 4];
        let queries = vec![
            Query {
                l: 5,
                r: 5,
                v: None,
            },
            Query {
                l: 5,
                r: 5,
                v: None,
            },
            Query {
                l: 3,
                r: 4,
                v: None,
            },
            Query {
                l: 1,
                r: 3,
                v: None,
            },
            Query {
                l: 1,
                r: 4,
                v: None,
            },
        ];

        let maximums = min_max(val, queries);

        assert_eq!(5, maximums.len());
        assert_eq!(4, maximums[0]);
        assert_eq!(4, maximums[1]);
        assert_eq!(3, maximums[2]);
        assert_eq!(4, maximums[3]);
        assert_eq!(4, maximums[4]);
    }
}
