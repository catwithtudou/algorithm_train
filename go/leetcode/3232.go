package leetcode

func canAliceWin(nums []int) bool {
	cnt := 0
	for _, x := range nums {
		if x < 10 {
			cnt += x
		} else {
			cnt -= x
		}
	}
	return cnt != 0
}
