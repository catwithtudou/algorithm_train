package leetcode

func maxSatisfied(customers []int, grumpy []int, minutes int) int {
	s := [2]int{}
	maxS1 := 0
	for i, v := range customers {
		s[grumpy[i]] += v
		if i < minutes-1 {
			continue
		}
		maxS1 = max(maxS1, s[1])
		if grumpy[i-minutes+1] > 0 {
			s[1] -= customers[i-minutes+1]
		}
	}

	return s[0] + maxS1
}
