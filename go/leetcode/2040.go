package leetcode

import (
	"slices"
	"sort"
)

func kthSmallestProduct(nums1 []int, nums2 []int, K int64) int64 {
	n, m, k := len(nums1), len(nums2), int(K)
	i0 := sort.SearchInts(nums1, 0)
	j0 := sort.SearchInts(nums2, 0)

	corners := []int{nums1[0] * nums2[0], nums1[0] * nums2[m-1], nums1[n-1] * nums2[0], nums1[n-1] * nums2[m-1]}
	left, right := slices.Min(corners), slices.Max(corners)

	ans := left + sort.Search(right-left, func(mx int) bool {
		mx += left
		cnt := 0

		if mx < 0 {
			i, j := 0, j0
			for i < i0 && j < m {
				if nums1[i]*nums2[j] > mx {
					j++
				} else {
					cnt += m - j
					i++
				}
			}

			i, j = i0, 0
			for i < n && j < j0 {
				if nums1[i]*nums2[j] > mx {
					i++
				} else {
					cnt += n - i
					j++
				}
			}
		} else {
			cnt = i0*(m-j0) + (n-i0)*j0

			i, j := 0, j0-1
			for i < i0 && j >= 0 {
				if nums1[i]*nums2[j] > mx {
					i++
				} else {
					cnt += i0 - i
					j--
				}
			}

			i, j = i0, m-1
			for i < n && j >= j0 {
				if nums1[i]*nums2[j] > mx {
					j--
				} else {
					cnt += j - j0 + 1
					i++
				}
			}

		}
		return cnt >= k
	})

	return int64(ans)
}
