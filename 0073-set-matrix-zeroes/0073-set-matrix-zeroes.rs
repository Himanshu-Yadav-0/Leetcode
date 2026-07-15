impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut pairs: Vec<(i32,i32)> = Vec::new();
        for (row_idx, row_elem) in matrix.iter().enumerate() {
            for (col_idx, col_elem) in row_elem.iter().enumerate() {
                if matrix[row_idx][col_idx] == 0{
                    pairs.push((row_idx as i32 ,col_idx as i32));
                }
            }
        }
        for pair in &pairs{
            for (row_idx, row_elem) in matrix.iter_mut().enumerate() {
                for (col_idx, col_elem) in row_elem.iter_mut().enumerate() {
                    if row_idx as i32 == pair.0 || col_idx as i32 == pair.1 {
                                    *col_elem = 0;
                                }
                }
            }
        }
    }
}