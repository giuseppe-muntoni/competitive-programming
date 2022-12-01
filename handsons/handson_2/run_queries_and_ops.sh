cargo run --release --bin queries_and_ops < queries_and_ops/test_set/input0.txt | diff - queries_and_ops/test_set/output0.txt
cargo run --release --bin queries_and_ops < queries_and_ops/test_set/input1.txt | diff - queries_and_ops/test_set/output1.txt
cargo run --release --bin queries_and_ops < queries_and_ops/test_set/input2.txt | diff - queries_and_ops/test_set/output2.txt