package leetcode

func countCommasII(n int64) (ans int64) {
	for i := int64(1000); i <= n; i *= 1000 {
		ans += n - i + 1
	}
	return
}
