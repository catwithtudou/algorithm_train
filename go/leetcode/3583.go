package leetcode

func specialTriplets(nums []int) (ans int) {
	const mod = 1000000007

	suf := make(map[int]int)
	for _, v := range nums {
		suf[v]++
	}

	pre := make(map[int]int)
	for _, x := range nums {
		suf[x]--
		ans += pre[x*2] * suf[x*2]
		pre[x]++
	}

	return ans % mod
}
