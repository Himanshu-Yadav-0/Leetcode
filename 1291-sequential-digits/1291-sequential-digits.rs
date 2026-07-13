impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let ll = low.to_string().len();
        let hl = high.to_string().len();

        let master: String = "123456789".to_string();
        let mut ans: Vec<i32> = Vec::new();
        for ws in ll..hl+1{
            let mut l =0;
            let mut r = ws -1 ;
            while r < master.len(){
                let num = &master[l..r+1].parse().unwrap();
                if *num <= high && *num >= low{
                    ans.push(*num);
                }
                l+=1;
                r+=1;
            }
        }
        ans
    }
}