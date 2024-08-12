package leetcode

type trie struct {
	children   [26]*trie
	isFinished bool
}

type MagicDictionary struct {
	*trie
}

func ConstructorMagicDictionary() MagicDictionary {
	return MagicDictionary{&trie{}}
}

func (d *MagicDictionary) BuildDict(dictionary []string) {
	for _, word := range dictionary {
		cur := d.trie
		for _, c := range word {
			c -= 'a'
			if cur.children[c] == nil {
				cur.children[c] = &trie{}
			}
			cur = cur.children[c]
		}
		cur.isFinished = true
	}
}

func dfs(node *trie, searchWord string, modified bool) bool {
	if searchWord == "" {
		return modified && node.isFinished
	}
	c := searchWord[0] - 'a'
	if node.children[c] != nil && dfs(node.children[c], searchWord[1:], modified) {
		return true
	}
	if !modified {
		for i, child := range node.children {
			if i != int(c) && child != nil && dfs(child, searchWord[1:], true) {
				return true
			}
		}
	}
	return false
}

func (d *MagicDictionary) Search(searchWord string) bool {
	return dfs(d.trie, searchWord, false)
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * obj := Constructor();
 * obj.BuildDict(dictionary);
 * param_2 := obj.Search(searchWord);
 */
