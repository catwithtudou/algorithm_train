package leetcode

import (
	"slices"
	"strings"
)

type trieNode struct {
	son     map[string]*trieNode
	name    string
	deleted bool
}

func deleteDuplicateFolder(paths [][]string) (ans [][]string) {

	root := &trieNode{}
	for _, path := range paths {
		cur := root
		for _, s := range path {
			if cur.son == nil {
				cur.son = make(map[string]*trieNode)
			}
			if cur.son[s] == nil {
				cur.son[s] = &trieNode{}
			}
			cur = cur.son[s]
			cur.name = s
		}
	}

	exprToNode := make(map[string]*trieNode)
	var genExpr func(*trieNode) string

	genExpr = func(node *trieNode) string {
		if node.son == nil {
			return node.name
		}

		expr := make([]string, 0, len(node.son))
		for _, son := range node.son {
			expr = append(expr, "("+genExpr(son)+")")
		}
		slices.Sort(expr)

		subTreeexpr := strings.Join(expr, "")
		n := exprToNode[subTreeexpr]
		if n != nil {
			n.deleted = true
			node.deleted = true
		} else {
			exprToNode[subTreeexpr] = node
		}
		return node.name + subTreeexpr
	}

	for _, son := range root.son {
		genExpr(son)
	}

	path := []string{}
	var dfs func(*trieNode)
	dfs = func(node *trieNode) {
		if node.deleted {
			return
		}
		path = append(path, node.name)
		ans = append(ans, slices.Clone(path))
		for _, son := range node.son {
			dfs(son)
		}
		path = path[:len(path)-1]
	}
	for _, son := range root.son {
		dfs(son)
	}

	return
}
