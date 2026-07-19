impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut freq = vec![0; 26];
        for c in s.chars() {
            freq[((c as u8) - b'a') as usize] += 1;
        }
        let mut used = vec![false; 26];
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            let idx = c as u8 - b'a';
            freq[idx as usize] -= 1;
            if used[idx as usize] {
                continue;
            }

            while let Some(&top) = stack.last() {
                let top_idx = (top as u8 - b'a') as usize;

                if c < top && freq[top_idx] > 0 {
                    stack.pop();
                    used[top_idx] = false;
                } else {
                    break;
                }
            }
            stack.push(c);
            used[idx as usize] = true;
        }

        stack.into_iter().collect()
    }
}