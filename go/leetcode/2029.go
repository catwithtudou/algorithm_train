package leetcode

func checkStone(n int, cnt [3]int) bool {
	if cnt[1] == 0 {
		return false
	}
	cnt[1]--
	rounds := 1 + min(cnt[1], cnt[2])*2 + cnt[0]
	if cnt[1] > cnt[2] {
		rounds++
	}

	return rounds < n && rounds%2 > 0
}

func stoneGameIX(stones []int) bool {
	cnt := [3]int{}
	for _, x := range stones {
		cnt[x%3]++
	}
	n := len(stones)
	return checkStone(n, cnt) || checkStone(n, [3]int{cnt[0], cnt[2], cnt[1]})
}
