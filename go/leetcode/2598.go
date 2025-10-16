package leetcode

func findSmallestInteger(nums []int, value int) (mex int) {

	cntMap := make(map[int]int)

	for _, num := range nums {
		cntMap[(num%value+value)%value]++
	}

	for cntMap[mex%value] > 0 {
		cntMap[mex%value]--
		mex++
	}

	return
}
