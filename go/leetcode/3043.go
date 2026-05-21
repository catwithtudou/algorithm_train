package leetcode

import "strconv"

func longestCommonPrefix(arr1 []int, arr2 []int) (ans int) {
	hang := make(map[string]bool)
	for _, v := range arr1 {
		arr := strconv.Itoa(v)
		for i := 1; i <= len(arr); i++ {
			hang[arr[:i]] = true
		}
	}

	for _, v := range arr2 {
		arr := strconv.Itoa(v)
		for i := 1; i <= len(arr) && hang[arr[:i]]; i++ {
			ans = max(ans, i)
		}
	}

	return
}
