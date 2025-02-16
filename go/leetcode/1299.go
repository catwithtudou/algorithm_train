package leetcode

func replaceElements(arr []int) []int {
	n := len(arr)
	pre := -1
	for i := n - 1; i >= 0; i-- {
		temp := arr[i]
		arr[i] = pre
		pre = max(pre, temp)
	}

	return arr
}
