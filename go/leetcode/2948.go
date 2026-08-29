package leetcode

import "sort"

func lexicographicallySmallestArray(nums []int, limit int) []int {
	n := len(nums)
	ans := make([]int, n)

	arr := make([][2]int, n)
	for i, x := range nums {
		arr[i] = [2]int{x, i}
	}

	sort.Slice(arr, func(i, j int) bool {
		return arr[i][0] < arr[j][0]
	})

	values := make([]int, n)
	indices := make([]int, n)

	for i, p := range arr {
		values[i] = p[0]
		indices[i] = p[1]
	}

	i := 0

	for i < n {
		start := i

		groupIndices := []int{}

		groupValues := []int{}

		for i < n && (i == start || values[i]-values[i-1] <= limit) {
			groupIndices = append(groupIndices, indices[i])
			groupValues = append(groupValues, values[i])
			i++
		}

		sort.Ints(groupIndices)

		for k := 0; k < len(groupIndices); k++ {
			ans[groupIndices[k]] = groupValues[k]
		}
	}

	return ans
}
