impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();

        // Flatten the grid
        let mut flatten = Vec::with_capacity(m * n);

        for row in &grid {
            for &x in row {
                flatten.push(x);
            }
        }

        let len = flatten.len();
        let k = (k as usize) % len;

        // If no shift is needed
        if k == 0 {
            return grid;
        }

        // Rotate right by k using reversals
        Self::reverse(&mut flatten, 0, len - 1);
        Self::reverse(&mut flatten, 0, k - 1);
        Self::reverse(&mut flatten, k, len - 1);

        // Convert back to 2D grid
        let mut res = Vec::with_capacity(m);

        let mut i = 0;
        while i < len {
            res.push(flatten[i..i + n].to_vec());
            i += n;
        }

        res
    }

    fn reverse(nums: &mut Vec<i32>, mut left: usize, mut right: usize) {
        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}