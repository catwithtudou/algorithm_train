package leetcode

func occurrencesOfElement(nums []int, queries []int, x int) []int {
	record := make(map[int]int, len(nums))
	cnt := 0
	for i, v := range nums {
		if v == x {
			cnt++
			record[cnt] = i
		}
	}
	ans := make([]int, len(queries))
	for i, v := range queries {
		ans[i] = -1
		if v, ok := record[v]; ok {
			ans[i] = v
		}
	}
	return ans
}
