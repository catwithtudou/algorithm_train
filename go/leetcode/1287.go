package leetcode

func findSpecialInteger(arr []int) int {
	n := len(arr)
	for i := 0; i < n; i++ {
		if i+n/4 < n && arr[i] == arr[i+n/4] {
			return arr[i]
		}
	}
	return 0
}
