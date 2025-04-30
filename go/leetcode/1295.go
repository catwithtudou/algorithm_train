package leetcode

func findNumbers(nums []int) (ans int) {

	for _, v := range nums {
		if v < 10 {
			continue
		}
		if v/10 > 0 && v/10 < 10 {
			ans++
			continue
		} else if v/1000 > 0 && v/1000 < 10 {
			ans++
			continue
		} else if v/100000 > 0 && v/100000 < 10 {
			ans++
			continue
		}
	}

	return
}
