package leetcode

type Node struct {
	Val   int
	Left  *Node
	Right *Node
	Next  *Node
}

// BFS
func connect(root *Node) *Node {
	if root == nil {
		return nil
	}
	list := []*Node{root}
	for len(list) > 0 {
		tmp := list
		list = nil
		for i, node := range tmp {
			if i+1 < len(tmp) {
				node.Next = tmp[i+1]
			}
			if node.Left != nil {
				list = append(list, node.Left)
			}
			if node.Right != nil {
				list = append(list, node.Right)
			}
		}
	}
	return root
}

// BFS+链表
func connect_One(root *Node) *Node {
	start := root
	for start != nil {
		var nextStart, last *Node
		handleNext := func(cur *Node) {
			if cur == nil {
				return
			}
			if nextStart == nil {
				nextStart = cur
			}
			if last != nil {
				last.Next = cur
			}
			last = cur
		}
		for q := start; q != nil; q = q.Next {
			handleNext(q.Left)
			handleNext(q.Right)
		}
		start = nextStart
	}

	return root
}

// DFS
func connect_Two(root *Node) *Node {
	pre := []*Node{}
	var dfs func(*Node, int)
	dfs = func(node *Node, depth int) {
		if node == nil {
			return
		}
		if depth == len(pre) { // 最左侧节点
			pre = append(pre, node)
		} else {
			pre[depth].Next = node
			pre[depth] = node
		}
		dfs(node.Left, depth+1)
		dfs(node.Right, depth+1)
	}
	dfs(root, 0)
	return root
}
