package leetcode

func distributeCandies(candies int, numPeople int) []int {
	ans := make([]int, numPeople)
	for i := 1; candies > 0; i++ {
		ans[(i-1)%numPeople] += min(i, candies)
		candies -= i
	}
	return ans
}
