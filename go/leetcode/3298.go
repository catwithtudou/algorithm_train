package leetcode

func validSubstringCountII(word1 string, word2 string) (ans int64) {
	if len(word1) < len(word2) {
		return 0
	}

	diff := [26]int{}
	for _, c := range word2 {
		diff[c-'a']++
	}

	less := 0
	for _, c := range diff {
		if c > 0 {
			less++
		}
	}

	left := 0
	for _, c := range word1 {
		diff[c-'a']--
		if diff[c-'a'] == 0 {
			less--
		}
		for less == 0 {
			if diff[word1[left]-'a'] == 0 {
				less++
			}
			diff[word1[left]-'a']++
			left++
		}

		ans += int64(left)
	}

	return ans
}
