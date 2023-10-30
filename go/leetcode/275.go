package leetcode

// 贪心
//func hIndex(citations []int) int {
//	n := len(citations)
//	res := 0
//	for i := 0; i < n; i++ {
//		if citations[i] >= n-i {
//			res = max(res, n-i)
//		}
//	}
//	return res
//}

// 二分查找
//func hIndex(citations []int) int {
//	n := len(citations)
//	res := n - sort.Search(n, func(i int) bool {
//		return citations[i] >= n-i // true 即在右边
//	})
//	return res
//}

func max(i, j int) int {
	if i > j {
		return i
	}
	return j
}
