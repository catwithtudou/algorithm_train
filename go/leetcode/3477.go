package leetcode

func numOfUnplacedFruits(fruits []int, baskets []int) int {
	n, cnt := len(fruits), 0
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if fruits[i] <= baskets[j] {
				baskets[j] = 0
				cnt++
				break
			}
		}
	}
	return n - cnt
}
