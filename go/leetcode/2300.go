package leetcode

import "sort"

// 二分查找+排序
func successfulPairs(spells []int, potions []int, success int64) []int {
	sort.Ints(potions)
	pNum := len(potions)
	for i := 0; i < len(spells); i++ {
		ans := sort.Search(len(potions), func(j int) bool {
			if int64(potions[j])*int64(spells[i]) >= success {
				return true
			}
			return false
		})
		spells[i] = pNum - ans
	}

	return spells
}

// 双指针+排序
func successfulPairsOther(spells []int, potions []int, success int64) []int {
	res := make([]int, len(spells))
	idx := make([]int, len(spells))
	for i, _ := range idx {
		idx[i] = i
	}

	sort.Slice(potions, func(i, j int) bool {
		return potions[i] > potions[j]
	})

	sort.Slice(idx, func(i, j int) bool {
		return spells[idx[i]] < spells[idx[j]]
	})

	j := 0
	for _, v := range idx {
		tmp := spells[v]
		for j < len(potions) && int64(tmp)*int64(potions[j]) >= success {
			j++
		}
		res[v] = j
	}

	return res
}
