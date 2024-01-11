package leetcode

func addMinimum(word string) int {
	n := len(word)
	d := make([]int, n+1)
	for i := 1; i <= n; i++ {
		d[i] = d[i-1] + 2
		if i > 1 && word[i-1] > word[i-2] {
			d[i] = d[i-1] - 1
		}
	}
	return d[n]
}
