package leetcode

func minDeletionSize(strs []string) (ans int) {
	m, n := len(strs), len(strs[0])
	for i := range n {
		for j := range m - 1 {
			if strs[j][i] > strs[j+1][i] {
				ans++
				break
			}
		}
	}
	return
}
