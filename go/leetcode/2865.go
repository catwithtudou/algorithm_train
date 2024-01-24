package leetcode

func maximumSumOfHeights(maxHeights []int) int64 {
	n := len(maxHeights)
	prefix := make([]int, n)
	suffix := make([]int, n)
	var stackPre, stackSuf []int
	res := 0

	for i := 0; i < n; i++ {
		for len(stackPre) > 0 && maxHeights[i] < maxHeights[stackPre[len(stackPre)-1]] {
			stackPre = stackPre[:len(stackPre)-1]
		}
		if len(stackPre) == 0 {
			prefix[i] = (i + 1) * maxHeights[i]
		} else {
			last := stackPre[len(stackPre)-1]
			prefix[i] = prefix[last] + (i-last)*maxHeights[i]
		}
		stackPre = append(stackPre, i)
	}

	for i := n - 1; i >= 0; i-- {
		for len(stackSuf) > 0 && maxHeights[i] < maxHeights[stackSuf[len(stackSuf)-1]] {
			stackSuf = stackSuf[:len(stackSuf)-1]
		}
		if len(stackSuf) == 0 {
			suffix[i] = (n - i) * maxHeights[i]
		} else {
			last := stackSuf[len(stackSuf)-1]
			suffix[i] = suffix[last] + (last-i)*maxHeights[i]
		}
		stackSuf = append(stackSuf, i)
		res = max(res, prefix[i]+suffix[i]-maxHeights[i])
	}

	return int64(res)
}

func maximumSumOfHeightsV0(maxHeights []int) (res int64) {
	n := len(maxHeights)
	for i := 0; i < n; i++ {
		pre, pSum := maxHeights[i], int64(maxHeights[i])
		for j := i - 1; j >= 0; j-- {
			pre = min(pre, maxHeights[j])
			pSum += int64(pre)
		}
		temp := maxHeights[i]
		for j := i + 1; j < n; j++ {
			temp = min(temp, maxHeights[j])
			pSum += int64(temp)
		}
		res = maxInt64(res, pSum)
	}

	return res
}
