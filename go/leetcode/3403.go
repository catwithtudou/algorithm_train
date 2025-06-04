package leetcode

func answerString(word string, numFriends int) (ans string) {
	if numFriends == 1 {
		return word
	}

	n := len(word)
	for i := range n {
		str := word[i:min(i+n-numFriends+1, n)]
		if ans < str {
			ans = str
		}
	}

	return
}
