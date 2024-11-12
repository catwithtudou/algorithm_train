package leetcode

func countKConstraintSubstrings(s string, k int) int {
	cnt := [2]int{}
	left, ans := 0, 0
	for i, c := range s {
		cnt[c&1]++
		for cnt[0] > k && cnt[1] > k {
			cnt[s[left]&1]--
			left++
		}
		ans += i - left + 1
	}

	return ans
}
