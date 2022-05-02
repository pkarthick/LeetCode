struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut s = 0;
        let mut e = nums.len() - 1;
        let mut nums = nums;

        loop {
            if s >= e {
                break;
            }

            if nums[s] & 1 == 1 {
                if nums[e] & 1 == 0 {
                    let t = nums[s];
                    nums[s] = nums[e];
                    nums[e] = t;
                    s += 1;
                }
                e -= 1;
            } else {
                s += 1;
            }
        }

        return nums;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn is_sorted_by_parity(v: Vec<i32>) -> bool {
        let count = v.iter().take_while(|n| **n % 2 == 0).count();
        v.iter().skip(count).all(|n| n % 2 == 1)
    }

    #[test]
    fn it_works() {
        assert!(is_sorted_by_parity(Solution::sort_array_by_parity(vec![3, 1, 2, 4])));
        assert!(is_sorted_by_parity(Solution::sort_array_by_parity(vec![0])));
        assert!(is_sorted_by_parity(Solution::sort_array_by_parity(vec![1,0,3])));
    }
}
