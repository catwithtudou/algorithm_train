package leetcode

import "slices"

func maximumBeautyII(flowers []int, newFlowers int64, target int, full int, partial int) int64 {
	n := len(flowers)

	leftF := int(newFlowers) - target*n
	for i, f := range flowers {
		flowers[i] = min(f, target)
		leftF += flowers[i]
	}

	if leftF == int(newFlowers) {
		return int64(n * full)
	}

	if leftF >= 0 {
		return int64(max((target-1)*partial+(n-1)*full, n*full))
	}

	slices.Sort(flowers)

	var ans, preSum, j int

	for i := 1; i <= n; i++ {
		leftF += target - flowers[i-1]
		if leftF < 0 {
			continue
		}

		for j < i && flowers[j]*j <= preSum+leftF {
			preSum += flowers[j]
			j++
		}

		avg := (leftF + preSum) / j
		totalBeauty := avg*partial + (n-i)*full
		ans = max(ans, totalBeauty)
	}

	return int64(ans)
}
