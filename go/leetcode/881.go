package leetcode

import "sort"

func numRescueBoats(people []int, limit int) int {
	ans := 0
	l, h := 0, len(people)-1
	sort.Ints(people)
	for l <= h {
		if people[l]+people[h] > limit {
			h--
		} else {
			l++
			h--
		}
		ans++
	}
	return ans
}
