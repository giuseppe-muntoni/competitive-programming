cargo run --release --bin holiday_planning < holiday_planning/test_set/input0.txt | diff - holiday_planning/test_set/output0.txt
cargo run --release --bin holiday_planning < holiday_planning/test_set/input1.txt | diff - holiday_planning/test_set/output1.txt
cargo run --release --bin holiday_planning < holiday_planning/test_set/input2.txt | diff - holiday_planning/test_set/output2.txt
cargo run --release --bin holiday_planning < holiday_planning/test_set/input3.txt | diff - holiday_planning/test_set/output3.txt
cargo run --release --bin holiday_planning < holiday_planning/test_set/input4.txt | diff - holiday_planning/test_set/output4.txt