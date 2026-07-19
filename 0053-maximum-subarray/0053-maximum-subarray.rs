use std::cmp;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max_sum = nums[0];
        for i in nums{
            sum+=i;
            max_sum = cmp::max(sum,max_sum);
            if sum <0{
                sum = 0;
            }
            }
        max_sum
        }
}