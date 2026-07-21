use std::cmp;
impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        
        let one_count = s.chars().filter(|&c| c=='1').count();
        
        let chars: Vec<char> = s.chars().collect();
        
        let cnt = chars.len();
        
        let mut left_zero = 0;
        let mut right_zero = 0;
        let mut i = 0;
        
        let mut ans = one_count;
        while i < cnt{
            if chars[i] == '0'{
                left_zero +=1;
                i+=1;
            }
            else if chars[i] == '1' && left_zero>0{
                while i < cnt && chars[i]=='1'{
                    i+=1;
                }
                while i < cnt && chars[i]=='0' {
                    right_zero +=1;
                    i+=1;
                }
                if right_zero>0 {
                    ans = cmp::max(ans, one_count + left_zero + right_zero);
                    left_zero = right_zero;
                    right_zero = 0;
                }
            }else{
                i+=1;
            }
        }
        
        ans as i32
    }
}