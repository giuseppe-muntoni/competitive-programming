cargo run --release --bin xmas_lights < xmas_lights/test_set/input0.txt | diff - xmas_lights/test_set/output0.txt
cargo run --release --bin xmas_lights < xmas_lights/test_set/input1.txt | diff - xmas_lights/test_set/output1.txt
cargo run --release --bin xmas_lights < xmas_lights/test_set/input2.txt | diff - xmas_lights/test_set/output2.txt
cargo run --release --bin xmas_lights < xmas_lights/test_set/input3.txt | diff - xmas_lights/test_set/output3.txt
cargo run --release --bin xmas_lights < xmas_lights/test_set/input4.txt | diff - xmas_lights/test_set/output4.txt
cargo run --release --bin xmas_lights < xmas_lights/test_set/input5.txt | diff - xmas_lights/test_set/output5.txt
cargo run --release --bin xmas_lights < xmas_lights/test_set/input6.txt | diff - xmas_lights/test_set/output6.txt