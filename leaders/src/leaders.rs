pub fn find_leaders(v: Vec<i32>) -> Vec<i32> {
    if v.len() == 0 {
        Vec::new()
    } else {
        let mut index = v.len() - 1;
        let mut max = *(v.get(index).expect("Your code is wrong!"));
        let mut leaders:Vec<i32> = Vec::new();
        leaders.insert(0, max);
        
        index-=1;
        while index != 0 {
            let el = *(v.get(index).expect("Your code is wrong!"));
            if el >= max {
                max = el;
                leaders.insert(0, max);
            }
            index-=1;
        }

        leaders
    }
}