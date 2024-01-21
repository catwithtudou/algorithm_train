package leetcode

func splitArray(nums []int, k int) int {

	sum, maxNum := 0, 0
	for _, v := range nums {
		sum += v
		maxNum = max(maxNum, v)
	}

	left := maxNum
	right := sum
	for left < right {
		mid := left + (right-left)/2
		split := splitNum(nums, mid)
		if split > k {
			left = mid + 1
		} else {
			right = mid
		}

	}

	return left
}

func splitNum(nums []int, maxNum int) int {
	splits := 1
	curSum := 0
	for _, v := range nums {
		if (curSum + v) > maxNum {
			curSum = 0
			splits++
		}
		curSum += v
	}

	return splits
}
