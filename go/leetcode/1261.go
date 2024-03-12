package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
type FindElements map[int]bool

func FindElementsConstructor(root *TreeNode) FindElements {
	elements := make(map[int]bool)
	var dfs func(*TreeNode, int)
	dfs = func(root *TreeNode, val int) {
		if root == nil {
			return
		}
		elements[val] = true
		dfs(root.Left, 2*val+1)
		dfs(root.Right, 2*val+2)
	}

	dfs(root, 0)
	return elements
}

func (this FindElements) Find(target int) bool {
	return this[target]
}

/**
 * Your FindElements object will be instantiated and called as such:
 * obj := Constructor(root);
 * param_1 := obj.Find(target);
 */
