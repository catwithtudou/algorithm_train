package leetcode

func maxHeightOfTriangle(red int, blue int) int {
	cnt := [2]int{}
	for i := 1; ; i++ {
		cnt[i%2] += i
		if (cnt[0] > red || cnt[0] > blue) && (cnt[1] > red || cnt[1] > blue) {
			return i - 1
		}
	}
}
