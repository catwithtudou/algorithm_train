package leetcode

func countPermutations(complexity []int) int {
	const mod = 1000000007
	ans := 1
	for i := 1; i < len(complexity); i++ {
		if complexity[i] <= complexity[0] {
			return 0
		}
		ans = ans * i % mod
	}
	return ans
}
