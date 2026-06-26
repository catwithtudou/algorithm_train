package leetcode

func countMajoritySubarrays(nums []int, target int) (ans int) {

	for i := 0; i < len(nums); i++ {
		cnt := 0
		for j := i; j < len(nums); j++ {
			if nums[j] == target {
				cnt++
			} else {
				cnt--
			}
			if cnt > 0 {
				ans++
			}
		}
	}

	return
}
