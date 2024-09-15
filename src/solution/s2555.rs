use std::vec;

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let len = k + 1;
        let mut res = 0;
        let mut max_count = vec![0; prize_positions.len()];

        let mut left = prize_positions.len() as i32 - 1;
        let mut right = left;

        while left >= 0 {
            while right >= 0
                && prize_positions[right as usize] - prize_positions[left as usize] >= len
            {
                right -= 1;
            }
            max_count[left as usize] = right - left + 1;
            res = res.max(max_count[left as usize]);
            if right != prize_positions.len() as i32 - 1 {
                res = res.max(max_count[right as usize + 1] + max_count[left as usize]);
            }

            if left != prize_positions.len() as i32 - 1 {
                max_count[left as usize] =
                    max_count[left as usize].max(max_count[left as usize + 1]);
            }

            left -= 1;
        }

        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn s2555_test1() {
        let vec = vec![1, 1, 2, 2, 3, 3, 5];
        let k = 2;
        assert!(Solution::maximize_win(vec, k) == 7);
    }
}
