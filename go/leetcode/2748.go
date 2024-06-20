package leetcode

func countBeautifulPairs(nums []int) int {
	cnt := [10]int{}
	ans := 0
	for _, x := range nums {
		for i := 1; i < 10; i++ {
			if cnt[i] > 0 && gcdDp(x%10, i) == 1 {
				ans += cnt[i]
			}
		}
		for x >= 10 {
			x /= 10
		}
		cnt[x]++
	}
	return ans
}

func gcdDp(a, b int) int {
	for a != 0 {
		a, b = b%a, a
	}
	return b
}
