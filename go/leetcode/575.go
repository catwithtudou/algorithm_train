package leetcode

func distributeCandies575(candyType []int) int {
	n := len(candyType)
	all := n / 2
	typeNum := 0
	typeMap := make(map[int]bool, all)
	for _, v := range candyType {
		if _, ok := typeMap[v]; !ok {
			typeNum++
			typeMap[v] = true
		}
	}

	return min(all, typeNum)
}
