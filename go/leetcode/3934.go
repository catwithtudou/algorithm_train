package leetcode

import (
	"math/rand"
	"sort"
)

func smallestUniqueSubarray(nums []int) int {
	n := len(nums)

	const mod = 1_070_777_777

	base := 9e8 - rand.Intn(1e8)

	powBase := make([]int, n+1)
	preHash := make([]int, n+1)
	powBase[0] = 1

	for i, x := range nums {
		powBase[i+1] = powBase[i] * base % mod
		preHash[i+1] = (preHash[i]*base + x) % mod
	}

	subHash := func(l, r int) int {
		return ((preHash[r]-preHash[l]*powBase[r-l])%mod + mod) % mod
	}

	ans := sort.Search(n, func(length int) bool {
		cnt := map[int]int{}
		for i := length; i <= n; i++ {
			h := subHash(i-length, i)
			cnt[h]++
		}

		for _, c := range cnt {
			if c == 1 {
				return true
			}
		}
		return false
	})

	return ans
}
