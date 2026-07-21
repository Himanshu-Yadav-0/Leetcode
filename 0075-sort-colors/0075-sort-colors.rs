impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut low = 0 as usize;
        let mut mid = 0 as usize;
        let mut high = nums.len();

        if high == 0{
            return ();
        }
        
        while mid < high {
            println!("{} {} {}", low, mid, high);
            if nums[mid] == 0 {
                nums.swap(mid, low);
                low += 1;
                mid += 1;
            } else if nums[mid] == 1 {
                mid += 1;
            } else {
                high -= 1;
                nums.swap(mid, high);
            }
        }
    }
}