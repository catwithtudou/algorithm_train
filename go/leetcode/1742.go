package leetcode

func countBalls(lowLimit int, highLimit int) (ans int) {
	cnt := [46]int{}
	for i := lowLimit; i <= highLimit; i++ {
		sum := 0
		for x := i; x > 0; x /= 10 {
			sum += x % 10
		}
		cnt[sum]++
		if cnt[sum] > ans {
			ans = cnt[sum]
		}
	}

	return ans
}
