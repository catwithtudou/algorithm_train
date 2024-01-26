package leetcode

import "math/bits"

type TreeAncestor [][]int

func TreeAncestorConstructor(n int, parent []int) TreeAncestor {
	m := bits.Len(uint(n))
	p := make([][]int, n)
	for i, v := range parent {
		p[i] = make([]int, m)
		p[i][0] = v
	}

	for i := 0; i < m-1; i++ {
		for x := 0; x < n; x++ {
			if num := p[x][i]; num != -1 {
				p[x][i+1] = p[num][i]
			} else {
				p[x][i+1] = -1
			}
		}
	}

	return p
}

func (t TreeAncestor) GetKthAncestor(node int, k int) int {
	m := bits.Len(uint(k))
	for i := 0; i < m; i++ {
		if k>>i&1 > 0 {
			node = t[node][i]
			if node == -1 {
				break
			}
		}
	}

	return node
}
