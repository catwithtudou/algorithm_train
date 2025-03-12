package leetcode

import "strings"

func countOfSubstrings(word string, k int) int {
	return countF(word, k) - countF(word, k+1)
}

func countF(word string, k int) (ans int) {

	cnt1 := map[byte]int{}
	cnt2, left := 0, 0

	for _, b := range word {

		if strings.ContainsRune("aeiou", b) {
			cnt1[byte(b)]++
		} else {
			cnt2++
		}

		for len(cnt1) == 5 && cnt2 >= k {
			out := word[left]
			if strings.ContainsRune("aeiou", rune(out)) {
				cnt1[out]--
				if cnt1[out] == 0 {
					delete(cnt1, out)
				}
			} else {
				cnt2--
			}
			left++
		}

		ans += left
	}

	return ans
}
