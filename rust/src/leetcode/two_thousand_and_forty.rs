pub struct Solution;

impl Solution {
    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        let n = nums1.len();
        let m = nums2.len();
        let k = k as usize;

        // 找到第一个大于等于0的元素位置
        let i0 = nums1.binary_search(&0).unwrap_or_else(|x| x);
        let j0 = nums2.binary_search(&0).unwrap_or_else(|x| x);

        // 计算四个角的乘积
        let corners = vec![
            nums1[0] as i64 * nums2[0] as i64,
            nums1[0] as i64 * nums2[m - 1] as i64,
            nums1[n - 1] as i64 * nums2[0] as i64,
            nums1[n - 1] as i64 * nums2[m - 1] as i64,
        ];

        let left = *corners.iter().min().unwrap();
        let right = *corners.iter().max().unwrap();

        // 二分搜索
        let mut l = 0;
        let mut r = (right - left) as usize;

        while l < r {
            let mid = l + (r - l) / 2;
            let mx = left + mid as i64;
            let mut cnt = 0;

            if mx < 0 {
                let mut i = 0;
                let mut j = j0;
                while i < i0 && j < m {
                    if nums1[i] as i64 * nums2[j] as i64 > mx {
                        j += 1;
                    } else {
                        cnt += m - j;
                        i += 1;
                    }
                }

                i = i0;
                j = 0;
                while i < n && j < j0 {
                    if nums1[i] as i64 * nums2[j] as i64 > mx {
                        i += 1;
                    } else {
                        cnt += n - i;
                        j += 1;
                    }
                }
            } else {
                cnt = i0 * (m - j0) + (n - i0) * j0;

                let mut i = 0;
                let mut j = j0 as i32 - 1;
                while i < i0 && j >= 0 {
                    if nums1[i] as i64 * nums2[j as usize] as i64 > mx {
                        i += 1;
                    } else {
                        cnt += i0 - i;
                        j -= 1;
                    }
                }

                i = i0;
                j = m as i32 - 1;
                while i < n && j >= j0 as i32 {
                    if nums1[i] as i64 * nums2[j as usize] as i64 > mx {
                        j -= 1;
                    } else {
                        cnt += (j - j0 as i32 + 1) as usize;
                        i += 1;
                    }
                }
            }

            if cnt >= k {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        left + l as i64
    }
}
