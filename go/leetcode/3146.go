package leetcode

func findPermutationDifference(s string, t string) int {
	pos := [26]int{}
	for i, c := range s {
		pos[c-'a'] = i
	}
	ans := 0
	for i, c := range t {
		temp := i - pos[c-'a']
		if temp < 0 {
			temp = -temp
		}
		ans += temp
	}
	return ans
}
