package leetcode

func replaceNonCoprimes(nums []int) []int {
	st := nums[:0]
	for _, x := range nums {
		for len(st) > 0 && gcd(x, st[len(st)-1]) > 1 {
			x = lcm(x, st[len(st)-1])
			st = st[:len(st)-1]
		}
		st = append(st, x)
	}
	return st
}

func lcm(a, b int) int {
	return a / gcd(a, b) * b
}
