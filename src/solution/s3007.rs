pub struct Solution;

// @lc code=start

const M: i32 = 64;
impl Solution {
    pub fn find_maximum_number(k: i64, x: i32) -> i64 {
        let mut res: u64 = 0;
        for i in (0..M - 1).rev() {
            if Solution::check(res + u64::pow(2, i as u32), x, k as u64) {
                res += u64::pow(2, i as u32);
            }
        }
        res as i64
    }

    pub fn check(num: u64, x: i32, k: u64) -> bool {
        let mut current_num: u64 = 0;
        let mut kx = x;
        while kx < M {
            current_num += Solution::calculate(num, kx);
            if current_num > k {
                return false;
            }
            kx += x;
        }
        true
    }

    pub fn calculate(num: u64, x: i32) -> u64 {
        let base = 2;
        let power = u64::pow(base, x as u32);
        let mut res = (num + 1) / power;
        res *= power / 2;
        res += ((num + 1) % power).saturating_sub(power / 2);
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_caculate() {
        assert_eq!(Solution::calculate(3, 1), 2);
        assert_eq!(Solution::calculate(10, 2), 5);
    }

    #[test]
    fn test_check() {
        assert!(Solution::check(3, 1, 4));
        assert!(Solution::check(10, 2, 8));
    }

    #[test]
    fn test_find_maximum_number() {
        assert_eq!(Solution::find_maximum_number(9, 1), 6);
        assert_eq!(Solution::find_maximum_number(7, 2), 9);
    }
}
