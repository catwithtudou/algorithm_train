package leetcode

func minDeletionSize(strs []string) (ans int) {
	n, m := len(strs), len(strs[0])
	a := make([]string, n)
next:
	for j := range m {
		for i := range n - 1 {
			if a[i]+string(strs[i][j]) > a[i+1]+string(strs[i+1][j]) {
				ans++
				continue next
			}
		}

		for i, s := range strs {
			a[i] += string(s[j])
		}
	}

	return ans
}
