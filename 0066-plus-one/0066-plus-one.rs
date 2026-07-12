impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = digits;
        for digit in res.iter_mut().rev(){
            if *digit < 9{
                *digit+=1;
                return res;
            }
            *digit = 0
        }
        res.insert(0, 1);
        res
    }
}