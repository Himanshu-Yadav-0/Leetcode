impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![vec![]; num_rows as usize];
        for i in 0..num_rows as usize{
            res[i] = vec![1; (i+1 )as usize];
            for j in 1..i as usize{
                res[i][j] = res[i-1][j] + res[i-1][j-1];
            }
        }
        res
    }
}