package leetcode

func getSneakyNumbers(nums []int) (ans []int) {

	numMap := make(map[int]bool)

	for _, num := range nums {
		if numMap[num] {
			ans = append(ans, num)
			if len(ans) == 2 {
				return ans
			}
		}
		numMap[num] = true
	}

	return
}
