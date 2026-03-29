package leetcode

func canBeEqual(s1 string, s2 string) bool {

	var cnt1, cnt2 [2][26]int

	for i, v := range s1 {
		cnt1[i%2][v-'a']++
		cnt2[i%2][s2[i]-'a']++
	}

	return cnt1 == cnt2
}
