package leetcode

func smallestNumber3345(n int, t int) int {
	for i := n; ; i++ {
		prod := 1
		for x := i; x > 0; x /= 10 {
			prod *= x % 10
		}
		if prod%t == 0 {
			return i
		}
	}
}
