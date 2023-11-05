package leetcode

import (
	"math/bits"

	"golang.org/x/exp/maps"
	"golang.org/x/exp/slices"
)

// 异或运算
func findMaximumXOR(nums []int) (ans int) {
	// 最高位
	highBit := bits.Len(uint(slices.Max(nums))) - 1
	visited := make(map[int]bool)
	mask := 0
	// 从最高位开始进行枚举
	for i := highBit; i >= 0; i-- {
		maps.Clear(visited)
		mask = mask | 1<<i
		newAns := ans | 1<<i
		for _, x := range nums {
			// 将小于i位的比特位置为0
			x = x & mask
			if visited[newAns^x] {
				ans = newAns
				break
			}
			visited[x] = true
		}
	}

	return ans
}

const highBit = 30

type trie struct {
	left  *trie
	right *trie
}

func (t *trie) add(num int) {
	cur := t
	for i := highBit; i >= 0; i-- {
		bit := num >> i & 1
		if bit == 0 {
			if cur.left == nil {
				cur.left = &trie{}
			}
			cur = cur.left
			continue
		}
		if cur.right == nil {
			cur.right = &trie{}
		}
		cur = cur.right
	}
}

func (t *trie) search(num int) (x int) {
	cur := t
	for i := highBit; i >= 0; i-- {
		bit := num >> i & 1
		if bit == 0 {
			if cur.right != nil {
				cur = cur.right
				x = x*2 + 1
			} else {
				cur = cur.left
				x *= 2
			}
			continue
		}

		if cur.left != nil {
			cur = cur.left
			x = x*2 + 1
		} else {
			cur = cur.right
			x *= 2
		}

	}
	return
}

// 字典树
func findMaximumXOR_One(nums []int) (ans int) {
	root := &trie{}
	for i := 1; i < len(nums); i++ {
		root.add(nums[i-1])
		ans = max(ans, root.search(nums[i]))
	}
	return
}
