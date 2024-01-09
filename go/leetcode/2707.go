package leetcode

import "math"

func minExtraChar(s string, dictionary []string) int {
	dMap := make(map[string]bool)
	for _, v := range dictionary {
		dMap[v] = true
	}
	sLen := len(s)
	dp := make([]int, sLen+1)
	for i := 1; i <= sLen; i++ {
		dp[i] = dp[i-1] + 1
		for j := 0; j < i; j++ {
			if dMap[s[j:i]] && dp[j] < dp[i] {
				dp[i] = dp[j]
			}
		}

	}
	return dp[sLen]
}

type Trie struct {
	children []*Trie
	isEnd    bool
}

func NewTrie() *Trie {
	return &Trie{
		children: make([]*Trie, 26),
		isEnd:    false,
	}
}

func insert(node *Trie, word string) {
	for i := len(word) - 1; i >= 0; i-- {
		ch := word[i] - 'a'
		if node.children[ch] == nil {
			node.children[ch] = NewTrie()
		}
		node = node.children[ch]
	}
	node.isEnd = true
}

func track(node *Trie, ch byte) (*Trie, bool) {
	if node == nil || node.children[ch-'a'] == nil {
		return nil, false
	}
	node = node.children[ch-'a']
	return node, node.isEnd
}

func minExtraCharByTrie(s string, dictionary []string) int {
	n := len(s)
	d := make([]int, n+1)
	for i := 1; i <= n; i++ {
		d[i] = math.MaxInt
	}
	trie := NewTrie()
	for _, e := range dictionary {
		insert(trie, e)
	}
	for i := 1; i <= n; i++ {
		d[i] = d[i-1] + 1
		node := trie
		for j := i - 1; j >= 0; j-- {
			var ok bool
			if node, ok = track(node, s[j]); ok && d[j] < d[i] {
				d[i] = d[j]
			}
		}
	}

	return d[n]
}
