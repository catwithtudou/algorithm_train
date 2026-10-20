package leetcode

func sortableIntegers(nums []int) (ans int) {
	n := len(nums)
	nextDec := make([]int, n) // nums[nextDec[i]] > nums[nextDec[i] + 1]
	nextDec[n-1] = n
	p := n
	for i := n - 2; i >= 0; i-- {
		if nums[i] > nums[i+1] {
			p = i
		}
		nextDec[i] = p
	}

	solve := func(k int) {
		lastMax := 0
		for r := k - 1; r < n; r += k {
			l := r - k + 1
			m := nextDec[l]
			if m >= r {
				if nums[l] < lastMax {
					return
				}
				lastMax = nums[r]
			} else {
				if nextDec[m+1] < r || nums[m+1] < lastMax || nums[r] > nums[l] {
					return
				}
				lastMax = nums[m]
			}
		}
		ans += k
	}

	for k := 1; k*k <= n; k++ {
		if n%k == 0 {
			solve(k)
			if k*k < n {
				solve(n / k)
			}
		}
	}

	return
}
