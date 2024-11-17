package leetcode

func numFriendRequests(ages []int) int {
	cnt := [121]int{}
	for _, age := range ages {
		cnt[age]++
	}

	ans := 0
	cntWindows, ageY := 0, 0
	for ageX, c := range cnt {
		cntWindows += c
		if ageY*2 <= ageX+14 {
			cntWindows -= cnt[ageY]
			ageY++
		}
		if cntWindows > 0 {
			ans += c*cntWindows - c
		}
	}
	return ans
}
