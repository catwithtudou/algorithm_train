package leetcode

func minOperations3542(nums []int) (ans int) {
	st := nums[:0]
	for _, x := range nums {
		for len(st) > 0 && x < st[len(st)-1] {
			st = st[:len(st)-1]
			ans++
		}
		if len(st) == 0 || x != st[len(st)-1] {
			st = append(st, x)
		}
	}
	if st[0] == 0 {
		ans--
	}
	return ans + len(st)
}
