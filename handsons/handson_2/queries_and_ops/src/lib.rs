pub struct Operation {
    pub l: usize,
    pub r: usize,
    pub v: i64,
}

pub struct Query {
    pub l: usize,
    pub r: usize,
}

fn prefix_sum(val: Vec<i64>) -> Vec<i64> {
    let mut res = 0;

    val.iter()
        .map(|x| {
            res += *x;
            res
        })
        .collect()
}

fn count_ops_occs(queries: Vec<Query>, ops_num: usize) -> Vec<i64> {
    let mut ops_counter: Vec<i64> = vec![0; ops_num];

    for i in 0..queries.len() {
        ops_counter[queries[i].l - 1] += 1;
        if queries[i].r < ops_num {
            ops_counter[queries[i].r] -= 1;
        }
    }

    prefix_sum(ops_counter)
}

pub fn queries_and_ops(mut val: Vec<i64>, ops: Vec<Operation>, queries: Vec<Query>) -> Vec<i64> {
    let ops_occs = count_ops_occs(queries, ops.len());

    let mut f: Vec<i64> = vec![0; val.len()];

    for i in 0..ops.len() {
        if ops_occs[i] == 0 {
            continue;
        }

        f[ops[i].l - 1] += ops[i].v * ops_occs[i];

        if ops[i].r < val.len() {
            f[ops[i].r] -= ops[i].v * ops_occs[i];
        }
    }

    f = prefix_sum(f);

    for i in 0..val.len() {
        val[i] += f[i];
    }

    val
}

#[cfg(test)]
mod tests {
    use crate::{queries_and_ops, Operation, Query};

    #[test]
    fn test_queries_and_ops_1() {
        let updated_vals = queries_and_ops(
            vec![1, 2, 3],
            vec![
                Operation { l: 1, r: 2, v: 1 },
                Operation { l: 1, r: 3, v: 2 },
                Operation { l: 2, r: 3, v: 4 },
            ],
            vec![
                Query { l: 1, r: 2 },
                Query { l: 1, r: 3 },
                Query { l: 2, r: 3 },
            ],
        );

        assert_eq!(vec![9, 18, 17], updated_vals);
    }
}
