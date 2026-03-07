package leetcode

func minOperations1758(s string) int {
	diff := 0
	for i, ch := range s {
		if int(ch-'0') != i%2 {
			diff++
		}
	}
	return min(diff, len(s)-diff)
}
