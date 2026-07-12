use std::collections::HashMap;
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut temp = arr.clone();
        temp.sort();
        let mut map: HashMap<i32,i32> = HashMap::new();
        let mut rank = 1;
        for elem in &temp{
            if !map.contains_key(elem){
            map.insert(*elem,rank);
            rank+=1;
            }
        }
        let mut res: Vec<i32> = Vec::new();
        for i in arr{
            let val = map.get(&i).unwrap();
            res.push(*val);
        }
        res
    }
}