package leetcode

func findEvenNumbers(digits []int) (ans []int) {

	cnt := [10]int{}
	for _, d := range digits {
		cnt[d]++
	}

next:
	for i := 100; i < 1000; i += 2 {
		c := [10]int{}
		for x := i; x > 0; x /= 10 {
			d := x % 10
			c[d]++
			if c[d] > cnt[d] {
				continue next
			}
		}
		ans = append(ans, i)
	}

	return
}
