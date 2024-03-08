package leetcode

func minimumPossibleSum(n int, target int) int {
	m := target / 2
	if n <= m {
		return ((1 + n) * n / 2) % 1_000_000_007
	}
	return (((1 + m) * m / 2) + ((target + target + (n - m) - 1) * (n - m) / 2)) % 1_000_000_007
}
