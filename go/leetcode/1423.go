package leetcode

func maxScore(cardPoints []int, k int) int {
	n := len(cardPoints)
	m := n - k
	s := 0
	for i := 0; i < m; i++ {
		s += cardPoints[i]
	}
	total, minS := s, s
	for i := m; i < n; i++ {
		total += cardPoints[i]
		s += cardPoints[i] - cardPoints[i-m]
		minS = min(minS, s)

	}
	return total - minS
}

func maxScoreOther(cardPoints []int, k int) int {
	n := len(cardPoints)
	s := 0
	for i := 0; i < k; i++ {
		s += cardPoints[i]
	}
	ans := s
	for i := 1; i <= k; i++ {
		s += cardPoints[n-i] - cardPoints[k-i]
		ans = max(ans, s)
	}
	return ans
}
