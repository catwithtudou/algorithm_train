package leetcode

func countPartitions3578(nums []int, k int) int {
	const mod = 1_000_000_007
	n := len(nums)
	var minQ, maxQ []int
	f := make([]int, n+1)
	f[0] = 1
	sumF, left := 0, 0

	for i, x := range nums {
		sumF += f[i]

		for len(minQ) > 0 && x <= nums[minQ[len(minQ)-1]] {
			minQ = minQ[:len(minQ)-1]
		}
		minQ = append(minQ, i)

		for len(maxQ) > 0 && x >= nums[maxQ[len(maxQ)-1]] {
			maxQ = maxQ[:len(maxQ)-1]
		}
		maxQ = append(maxQ, i)

		for nums[maxQ[0]]-nums[minQ[0]] > k {
			sumF -= f[left]
			left++
			if minQ[0] < left {
				minQ = minQ[1:]
			}
			if maxQ[0] < left {
				maxQ = maxQ[1:]
			}
		}

		f[i+1] = sumF % mod
	}

	return f[n]
}
