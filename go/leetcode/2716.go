package leetcode

func minimizedStringLength(s string) int {
	chMap := make(map[rune]struct{})
	for _, c := range s {
		chMap[c] = struct{}{}
	}
	return len(chMap)
}
