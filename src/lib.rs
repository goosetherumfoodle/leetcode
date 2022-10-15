mod problem_704 {
    // Binary Search
    // Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
    // You must write an algorithm with O(log n) runtime complexity.

    pub fn bin_search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len() - 1;
        let mut i;
        while start <= end {
            i = ((end - start) / 2) + start;
            match target.cmp(&nums[i]) {
                std::cmp::Ordering::Equal => {
                    return i as i32;
                }
                std::cmp::Ordering::Less => {
                    if i == 0 {
                        return -1;
                    }
                    end = i - 1;
                }
                std::cmp::Ordering::Greater => {
                    start = i + 1;
                }
            }
        }
        -1
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn bin_search_not_found() {
            let nums = vec![-1, 0, 3, 5, 9, 12];
            let target = 2;

            let result = super::bin_search(nums, target);

            let expected = -1;
            assert_eq!(result, expected);
        }

        #[test]
        fn bin_search_all() {
            let nums = vec![-1, 0, 3, 5, 9, 12, 20, 42, 100];
            for i in 0..(nums.len() - 1) {
                let target = nums[i];

                let result = super::bin_search(nums.clone(), target);

                assert_eq!(result, i as i32);
            }
        }

        #[test]
        fn regression() {
            let nums = vec![5];
            let target = -5;

            let result = super::bin_search(nums, target);

            let expected = -1;
            assert_eq!(result, expected);
        }
    }
}

mod problem_278 {
    // First Bad Version

    // You are a product manager and currently leading a team to develop a new product. Unfortunately, the latest version of your product fails the quality check. Since each version is developed based on the previous version, all the versions after a bad version are also bad.
    // Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one, which causes all the following ones to be bad.
    // You are given an API bool isBadVersion(version) which returns whether version is bad. Implement a function to find the first bad version. You should minimize the number of calls to the API.


    struct Solution {
        bad_version: i32,
    }

    impl Solution {
        #[allow(non_snake_case)]
        pub fn isBadVersion(&self, version: i32) -> bool {
            self.bad_version <= version
        }
    }

    impl Solution {
        pub fn first_bad_version(&self, n: i32) -> i32 {
            if self.isBadVersion(1) { return 1 }

            let mut last_bad = n;
            let mut last_good = 1;
            loop {
                if last_bad - last_good <= 1 { return last_bad; }

                let query = ((last_bad - last_good) / 2) + last_good;
                let is_bad = self.isBadVersion(query);
                if is_bad {
                    last_bad = query
                } else {
                    last_good = query
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use rand::Rng;

        #[test]
        fn test_floor() {
            let latest_version = 1;
            let bad_version = 1;
            let subj = super::Solution { bad_version };

            let result = subj.first_bad_version(latest_version);

            assert_eq!(result, bad_version);
        }

        #[test]
        fn test_reg() {
            let latest_version = 5;
            let bad_version = 5;
            let subj = super::Solution { bad_version };

            let result = subj.first_bad_version(latest_version);

            assert_eq!(result, bad_version);
        }


        #[test]
        fn test_again() {
            let latest_version = 2;
            let bad_version = 1;
            let subj = super::Solution { bad_version };

            let result = subj.first_bad_version(latest_version);

            assert_eq!(result, bad_version);
        }


        #[test]
        fn test_many() {
            let mut runs = 100;
            while runs >= 1 {
                runs -= 1;
                let latest_version = 100;
                let bad_version = rand::thread_rng().gen_range(1..=latest_version);
                let subj = super::Solution { bad_version };

                let result = subj.first_bad_version(latest_version);

                assert_eq!(result, bad_version,
                "\nLATEST_VERSION: {latest_version}\nBAD_VERSION: {bad_version}\n");
            }
        }

    }
}

mod problem_35 {
    // Search Insert Position
    // Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
    // You must write an algorithm with O(log n) runtime complexity.

    pub fn bin_search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len() - 1;
        let mut i;
        while start <= end {
            i = ((end - start) / 2) + start;
            match target.cmp(&nums[i]) {
                std::cmp::Ordering::Equal => {
                    return i as i32;
                }
                std::cmp::Ordering::Less => {
                    if i == 0 {
                        return 0;
                    }
                    end = i - 1;
                }
                std::cmp::Ordering::Greater => {
                    start = i + 1;
                }
            }
        }
        start as i32
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn example_1() {
            let nums = vec![1, 3, 5, 6];
            let target = 5;

            let result = super::bin_search(nums, target);

            let expected = 2;
            assert_eq!(result, expected);
        }

        #[test]
        fn example_2() {
            let nums = vec![1, 3, 5, 6];
            let target = 2;

            let result = super::bin_search(nums, target);

            let expected = 1;
            assert_eq!(result, expected);
        }

        #[test]
        fn example_3() {
            let nums = vec![1, 3, 5, 6];
            let target = 7;

            let result = super::bin_search(nums, target);

            let expected = 4;
            assert_eq!(result, expected);
        }

        #[test]
        fn example_4() {
            let nums = vec![5];
            let target = 5;

            let result = super::bin_search(nums, target);

            let expected = 0;
            assert_eq!(result, expected);
        }

        #[test]
        fn example_5() {
            let nums = vec![5, 8, 10];
            let target = 4;

            let result = super::bin_search(nums, target);

            let expected = 0;
            assert_eq!(result, expected);
        }

        #[test]
        fn test_bin_search_all() {
            let nums = vec![-1, 0, 3, 5, 9, 12, 20, 42, 100];
            for i in 0..(nums.len() - 1) {
                let target = nums[i];

                let result = super::bin_search(nums.clone(), target);

                assert_eq!(result, i as i32);
            }
        }
    }
}

mod problem_217 {
// Contains Duplicate
// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
    use std::collections::HashSet;

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        return HashSet::<_>::from_iter(nums.iter()).len() != nums.len();
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn example_1() {
            let input = vec![1,2,3,1];

            assert!(super::contains_duplicate(input));
        }

        #[test]
        fn example_2() {
            let input = vec![1,2,3,4];

            assert!(!super::contains_duplicate(input));
        }

        #[test]
        fn example_3() {
            let input = vec![1,1,1,3,3,4,3,2,4,2];

            assert!(super::contains_duplicate(input));
        }
    }
}

mod problem_977 {
    // Squares of a Sorted Array
    // Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.
    // Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.

    fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut out = vec![0; nums.len()];
        let mut l: i32 = 0;
        let mut r: i32 = (nums.len() - 1).try_into().unwrap();
        let mut i: i32 = (nums.len() - 1).try_into().unwrap();
        while 0 <= i {
            if nums[l as usize].pow(2) > nums[r as usize].pow(2) {
                out[i as usize] = nums[l as usize].pow(2);
                l += 1;
                i -= 1;
            } else {
                out[i as usize] = nums[r as usize].pow(2);
                r -= 1;
                i -= 1;
            }

        }
        out
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn example_1() {
            let nums = vec![-4,-1,0,3,10];

            let result = super::sorted_squares(nums);

            let expected = vec![0,1,9,16,100];
            assert_eq!(result, expected);
        }

        #[test]
        fn example_2() {
            let nums = vec![-7,-3,2,3,11];

            let result = super::sorted_squares(nums);

            let expected = vec![4,9,9,49,121];
            assert_eq!(result, expected);
        }

        #[test]
        fn example_3() {
            let nums = vec![0,3,10];

            let result = super::sorted_squares(nums);

            let expected = vec![0,9,100];
            assert_eq!(result, expected);
        }

        #[test]
        fn example_4() {
            let nums = vec![-3,-2,-1,0];

            let result = super::sorted_squares(nums);

            let expected = vec![0,1,4,9];
            assert_eq!(result, expected);
        }
    }
}

mod problem_283 {
    // Move Zeroes
    // Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
    // Note that you must do this in-place without making a copy of the array.

    fn move_zeros(nums: &mut Vec<i32>) {
        let mut first_nonzero: usize = nums.len() - 1;
        while 0 <= first_nonzero {
            if nums[first_nonzero] != 0 {
                first_nonzero -= 1;
            }
        }
    }

    fn walkback(nums: &mut Vec<i32>, start: usize, end: usize) {
        let mut start = start;
        while start < end {
            let tmp = nums[start + 1];
            nums[start + 1] = nums[start];
            nums[start] = tmp;
            start += 1;
        }
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn walkback() {
            let mut input = vec![1,2,3,4,5];

            super::walkback(&mut input, 1, 3);

            let expected = vec![1,3,4,2,5];
            assert_eq!(input, expected);
        }

        #[ignore]
        #[test]
        fn example_1() {
            let mut input = vec![0,1,0,3,12];

            super::move_zeros(&mut input);

            let expected = vec![1,3,12,0,0];
            assert_eq!(input, expected);
        }

        #[ignore]
        #[test]
        fn example_2() {
            let mut input = vec![0];

            super::move_zeros(&mut input);

            let expected = vec![0];
            assert_eq!(input, expected);
        }
    }
}

mod problem_53 {
    /* Maximum Subarray
    Medium


    Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

    A subarray is a contiguous part of an array.

    Constraints:
    1 <= nums.length <= 105
    -104 <= nums[i] <= 104
     */

    use std::cmp::max;

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut sum = 0;
        for n in nums.into_iter() {
            if sum <= 0 { sum = 0 }
            sum += n;
            max_sum = max(max_sum, sum);
        }
        return max_sum as i32;
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn example_1() {
            let nums = vec![-2,1,-3,4,-1,2,1,-5,4];

            let result = super::max_sub_array(nums);

            assert_eq!(result, 6);
        }

        #[test]
        fn example_2() {
            let nums = vec![1];

            let result = super::max_sub_array(nums);

            assert_eq!(result, 1);
        }

        #[test]
        fn example_3() {
            let nums = vec![5,4,-1,7,8];

            let result = super::max_sub_array(nums);

            assert_eq!(result, 23);
        }
    }
}

mod problem_189 {
    /*
    Rotate Array
Medium

Given an array, rotate the array to the right by k steps, where k is non-negative.
*/
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {

    }


     #[cfg(test)]
    mod tests {
        fn example_1() {
        /*
        Input: nums = [1,2,3,4,5,6,7], k = 3
Output: [5,6,7,1,2,3,4]
Explanation:
rotate 1 steps to the right: [7,1,2,3,4,5,6]
rotate 2 steps to the right: [6,7,1,2,3,4,5]
rotate 3 steps to the right: [5,6,7,1,2,3,4]
*/
            let mut nums = [1,2,3,4,5,6,7];
            let k = 3;

            rotate(nums, k);

            assert_eq!(nums, vec![5,6,7,1,2,3,4]);
        }

        fn example_2() {
/*
Input: nums = [-1,-100,3,99], k = 2
Output: [3,99,-1,-100]
Explanation:
rotate 1 steps to the right: [99,-1,-100,3]
rotate 2 steps to the right: [3,99,-1,-100]
*/
            let mut nums = [-1,-100,3,99];
            let k = 2;

            rotate(nums, k);

            assert_eq!(nums, vec![3,99,-1,-100]);
        }
    }
}
