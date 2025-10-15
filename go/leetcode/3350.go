package leetcode

func maxIncreasingSubarrays(nums []int) (ans int) {
	preCnt, cnt := 0, 0

	for i, x := range nums {
		cnt++
		if i == len(nums)-1 || x >= nums[i+1] {
			maxLen := max(min(preCnt, cnt), cnt/2)
			ans = max(ans, maxLen)
			preCnt = cnt
			cnt = 0
		}
	}

	return ans
}
