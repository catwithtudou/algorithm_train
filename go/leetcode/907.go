package leetcode

func sumSubarrayMins(arr []int) int {
	const mod int = 1e9 + 7
	n := len(arr)
	left, right := make([]int, n), make([]int, n)
	monoStack := make([]int, 0, n)
	for i, v := range arr {
		for len(monoStack) > 0 && v <= arr[monoStack[len(monoStack)-1]] {
			monoStack = monoStack[:len(monoStack)-1]
		}
		if len(monoStack) == 0 {
			left[i] = i + 1
		} else {
			left[i] = i - monoStack[len(monoStack)-1]
		}
		monoStack = append(monoStack, i)
	}
	monoStack = make([]int, 0, n)
	for i := n - 1; i >= 0; i-- {
		for len(monoStack) > 0 && arr[i] < arr[monoStack[len(monoStack)-1]] {
			monoStack = monoStack[:len(monoStack)-1]
		}
		if len(monoStack) == 0 {
			right[i] = n - i
		} else {
			right[i] = monoStack[len(monoStack)-1] - i
		}
		monoStack = append(monoStack, i)
	}

	ans := 0
	for i, v := range arr {
		ans = (ans + left[i]*right[i]*v) % mod
	}

	return ans
}
