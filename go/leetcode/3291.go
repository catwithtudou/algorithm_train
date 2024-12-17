package leetcode

type CharacterTrie struct {
	children [26]*CharacterTrie
}

func (t *CharacterTrie) insert(w string) {
	node := t
	for _, c := range w {
		idx := c - 'a'
		if node.children[idx] == nil {
			node.children[idx] = &CharacterTrie{}
		}
		node = node.children[idx]
	}
}

func minValidStrings(words []string, target string) int {
	t := &CharacterTrie{}
	for _, w := range words {
		t.insert(w)
	}

	inf := 1 << 30
	n := len(target)
	memo := make([]int, n)
	var dfs func(int) int
	dfs = func(i int) int {
		if i >= n {
			return 0
		}

		if memo[i] != 0 {
			return memo[i]
		}

		node := t
		memo[i] = inf
		for j := i; j < n; j++ {
			idx := target[j] - 'a'
			if node.children[idx] == nil {
				break
			}
			memo[i] = min(memo[i], dfs(j+1)+1)
			node = node.children[idx]
		}
		return memo[i]
	}

	if ans := dfs(0); ans < inf {
		return ans
	}
	return -1
}
