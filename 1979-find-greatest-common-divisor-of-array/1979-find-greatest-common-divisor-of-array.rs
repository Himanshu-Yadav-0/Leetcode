impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut smallest = nums[0];
        let mut largest = nums[0];
        for i in 0..nums.len(){
            if nums[i]<smallest{
                smallest = nums[i];
            }else if nums[i]>largest{
                largest = nums[i];
            }
        }
        Self::find_gcd_mine(smallest,largest)
    }

    pub fn find_gcd_mine(a: i32, b: i32) -> i32{
        let reminder = a % b;
        if reminder == 0{
            return b;
        }
        
        Self::find_gcd_mine(b, reminder)
    }
}