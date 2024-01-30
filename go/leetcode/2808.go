package leetcode

func minimumSeconds(nums []int) int {
	n := len(nums)
	mp := make(map[int][]int)
	for i, num := range nums {
		mp[num] = append(mp[num], i)
	}
	res := n
	for _, pos := range mp {
		mx := pos[0] + n - pos[len(pos)-1]
		for i := 1; i < len(pos); i++ {
			mx = max(mx, pos[i]-pos[i-1])
		}
		res = min(res, mx/2)
	}

	return res
}
