package leetcode

func missingRolls(rolls []int, mean int, n int) []int {
	rem := mean * (len(rolls) + n)
	for _, v := range rolls {
		rem -= v
	}
	if rem < n || rem > n*6 {
		return nil
	}
	ans := make([]int, n)
	avg, extra := rem/n, rem%n
	for i := 0; i < n; i++ {
		ans[i] = avg
		if i < extra {
			ans[i] += 1
		}
	}
	return ans
}
