package leetcode

func countLargestGroup(n int) (ans int) {
	cnt := make(map[int]int)
	maxCnt := 0
	for i := 1; i <= n; i++ {
		sum := 0
		for x := i; x > 0; x /= 10 {
			sum += x % 10
		}
		cnt[sum]++
		if cnt[sum] > maxCnt {
			maxCnt = cnt[sum]
			ans = 1
		} else if cnt[sum] == maxCnt {
			ans++
		}
	}

	return
}
