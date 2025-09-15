package leetcode

func canBeTypedWords(text string, brokenLetters string) (ans int) {
	cnt := [26]int{}
	for _, v := range brokenLetters {
		cnt[v-'a']++
	}

	isWord := true
	for _, v := range text {
		if v == ' ' {
			if isWord {
				ans++
			}
			isWord = true
			continue
		}
		if cnt[v-'a'] > 0 {
			isWord = false
		}
	}

	if isWord {
		ans++
	}

	return
}
