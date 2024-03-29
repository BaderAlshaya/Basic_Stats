// Copyright © 2019 Bader Alshaya
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///! Functions to compute various statistics on a slice of
///! floating-point numbers.

/// Type of statistics function. If the statistic
/// is ill-defined, `None` will be returned.
pub type StatFn = fn(&[f64]) -> Option<f64>;

/// Arithmetic mean of input values. The mean of an empty
/// list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[-1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(1.0), mean(&[1.0]));
/// ```
pub fn mean(nums: &[f64]) -> Option<f64> {
    if nums.is_empty() {
        Some(0.0)
    } else {
        let len = nums.len() as f64;
        let mut sum = 0.0;
        for i in &nums[..] {
            sum = sum + i;
        }
        Some(sum / len)
    }
}

/// Population standard deviation of input values. The
/// standard deviation of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, stddev(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), stddev(&[1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), stddev(&[1.0]));
/// ```
pub fn stddev(nums: &[f64]) -> Option<f64> {
    if nums.is_empty() {
        None
    } else if nums.len() == 1 {
        Some(0.0)
    } else {
        let avg = mean(nums).unwrap();
        let mut sums = Vec::new();
        for i in &nums[..] {
            sums.push((i - avg).powf(2.0));
        }
        Some(mean(&sums[..]).unwrap().sqrt())
    }
}

/// Median value of input values, taking the value closer
/// to the beginning to break ties. The median
/// of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, median(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), median(&[0.0, 0.5, -1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(5.0), median(&[5.0]));
/// ```
pub fn median(nums: &[f64]) -> Option<f64> {
    // Make a sorted copy of the input floats.
    let mut nums = nums.to_owned();
    // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838/2
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if nums.is_empty() {
        None
    } else {
        let mid = (nums.len() - 1) / 2;
        Some(nums[mid])
    }
}

/// L2 norm (Euclidean norm) of input values. The L2
/// norm of an empty list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), l2(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(5.0), l2(&[-3.0, 4.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(5.0), l2(&[5.0]));
/// ```
pub fn l2(nums: &[f64]) -> Option<f64> {
    if nums.is_empty() {
        Some(0.0)
    } else {
        let mut sum = 0.0;
        for i in &nums[..] {
            sum = sum + i.powf(2.0);
        }
        Some(sum.sqrt())
    }
}
