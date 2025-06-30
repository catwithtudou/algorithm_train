package leetcode

func findLHS(nums []int) (ans int) {
	cnt := map[int]int{}
	for _, v := range nums {
		cnt[v]++
	}

	for k, v := range cnt {
		if re, ok := cnt[k+1]; ok {
			ans = max(ans, v+re)
		}
	}
	return
}
