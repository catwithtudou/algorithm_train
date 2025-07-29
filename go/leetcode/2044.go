package leetcode

func countMaxOrSubsets(nums []int) (ans int) {

	totalOr := 0

	for _, v := range nums {
		totalOr |= v
	}

	var dfs func(int, int)
	dfs = func(i, or int) {
		if i == len(nums) {
			if or == totalOr {
				ans++
			}
			return
		}
		dfs(i+1, or|nums[i])
		dfs(i+1, or)
	}

	dfs(0, 0)

	return
}
