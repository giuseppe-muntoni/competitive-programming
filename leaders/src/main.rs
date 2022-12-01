mod leaders;

fn main() {
    let v: Vec<i32> = vec![16,17,4,3,5,2];
    let result = leaders::find_leaders(v);
    assert_eq!(result, vec![17,5,2]);
}
    
