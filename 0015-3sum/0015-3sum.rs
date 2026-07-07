impl Solution {
    pub fn three_sum(n: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = n.clone();
        nums.sort();
        let mut res: Vec<Vec<i32>> = vec![];
        if nums.len() < 3 {
            return res;
        }
        for i in 0..nums.len() - 2 {
            if nums[i] > 0 {
                break;
            }

            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = nums.len() - 1;
            let target = -nums[i];

            while l < r {
                let sum = nums[l] + nums[r];
                if sum == target {
                    res.push(vec![nums[i] as i32, nums[l] as i32, nums[r] as i32]);
                    l += 1;
                    r -= 1;
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1
                    }
                    while l < r && nums[r] == nums[r + 1] {
                        r -= 1
                    }
                } else if sum<target{
                    l+=1;
                }else{
                    r-=1;
                }
            }
        }
        res
    }
}