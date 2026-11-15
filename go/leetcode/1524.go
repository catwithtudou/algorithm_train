package leetcode

func numOfSubarrays(arr []int) (ans int) {
	sum := make([]int, len(arr)+1)
	for i, x := range arr {
		sum[i+1] = (sum[i] + x) % 2
	}

	cnt := [2]int{}
	for _, s := range sum {
		ans += cnt[s^1]
		cnt[s]++
	}

	return ans % (1e9 + 7)
}
