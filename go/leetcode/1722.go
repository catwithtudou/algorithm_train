package leetcode

type UnionFindDis struct {
	fa   []int
	rank []int
}

func NewUnionFindDis(n int) *UnionFindDis {
	fa := make([]int, n)
	rank := make([]int, n)
	for i := 0; i < n; i++ {
		fa[i] = i
	}
	return &UnionFindDis{fa: fa, rank: rank}
}

func (uf *UnionFindDis) find(x int) int {
	if uf.fa[x] != x {
		uf.fa[x] = uf.find(uf.fa[x])
	}
	return uf.fa[x]
}

func (uf *UnionFindDis) union(x, y int) {
	x = uf.find(x)
	y = uf.find(y)
	if x == y {
		return
	}
	if uf.rank[x] < uf.rank[y] {
		x, y = y, x
	}
	uf.fa[y] = x
	if uf.rank[x] == uf.rank[y] {
		uf.rank[x]++
	}
}

func minimumHammingDistance(source []int, target []int, allowedSwaps [][]int) int {
	n := len(source)
	uf := NewUnionFindDis(n)
	for _, pair := range allowedSwaps {
		uf.union(pair[0], pair[1])
	}

	sets := make(map[int]map[int]int)
	for i := 0; i < n; i++ {
		f := uf.find(i)
		if sets[f] == nil {
			sets[f] = make(map[int]int)
		}
		sets[f][source[i]]++
	}

	ans := 0
	for i := 0; i < n; i++ {
		f := uf.find(i)
		if sets[f][target[i]] > 0 {
			sets[f][target[i]]--
		} else {
			ans++
		}
	}
	return ans
}
