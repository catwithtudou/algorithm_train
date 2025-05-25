package leetcode

func longestPalindrome(words []string) (ans int) {
	cnt := [26][26]int{}
	for _, w := range words {
		cnt[w[0]-'a'][w[1]-'a']++
	}
	odd := 0
	for i := range cnt {
		c := cnt[i][i]
		ans += c - c%2
		odd |= c % 2
		for j := i + 1; j < 26; j++ {
			ans += min(cnt[i][j], cnt[j][i]) * 2
		}
	}
	return (ans + odd) * 2
}
