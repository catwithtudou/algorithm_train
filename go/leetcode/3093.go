package leetcode

import "math"

func stringIndices(wordsContainer, wordsQuery []string) []int {
	type node struct {
		son       [26]*node
		minLen    int
		bestIndex int
	}

	root := &node{minLen: math.MaxInt}

	for i, word := range wordsContainer {
		l := len(word)

		if l < root.minLen {
			root.minLen = l
			root.bestIndex = i
		}

		cur := root

		for j := l - 1; j >= 0; j-- {
			b := word[j] - 'a'
			if cur.son[b] == nil {
				cur.son[b] = &node{minLen: math.MaxInt}
			}
			cur = cur.son[b]
			if l < cur.minLen {
				cur.minLen = l
				cur.bestIndex = i
			}
		}
	}

	ans := make([]int, len(wordsQuery))

	for i, word := range wordsQuery {
		cur := root
		for j := len(word) - 1; j >= 0 && cur.son[word[j]-'a'] != nil; j-- {
			cur = cur.son[word[j]-'a']
		}
		ans[i] = cur.bestIndex
	}

	return ans
}
