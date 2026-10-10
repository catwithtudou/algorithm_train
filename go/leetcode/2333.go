package leetcode

import "slices"

func minSumSquareDiff(a []int, nums2 []int, k1 int, k2 int) int64 {
	ans, sum := 0, 0
	for i, v := range a {
		a[i] = abs(v - nums2[i])
		sum += a[i]
		ans += a[i] * a[i]
	}
	k := k1 + k2
	if sum <= k {
		return 0
	}

	slices.SortFunc(a, func(a, b int) int { return b - a })
	a = append(a, 0)

	for i, v := range a {
		i++
		ans -= v * v
		if c := i * (v - a[i]); c < k {
			k -= c
			continue
		}
		v -= k / i
		ans += k%i*(v-1)*(v-1) + (i-k%i)*v*v
		break
	}

	return int64(ans)
}
