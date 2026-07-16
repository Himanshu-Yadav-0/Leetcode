impl Solution {
        pub fn gcd_sum(nums: Vec<i32>) -> i64 {
            let mut max = nums[0];
            let mut prefix_gcd: Vec<i64> = Vec::new();
            let mut sum = 0;
            for i in &nums{
                if *i>max{
                    max = *i;
                }
                let current_gcd = Self::find_gcd(*i as i64, max as i64);
                prefix_gcd.push(current_gcd);
            }
            prefix_gcd.sort();
            let mut i = 0;
            let mut j = prefix_gcd.len() -1;
            while i<j{
                sum += Self::find_gcd(prefix_gcd[i], prefix_gcd[j]);
                i+=1;
                j-=1;
            }
            
            return sum as i64;
        }

    pub fn find_gcd(a: i64, b: i64) -> i64{
        let reminder = a % b;
        if reminder == 0{
            return b;
        }
        
        Self::find_gcd(b, reminder)
    }
}