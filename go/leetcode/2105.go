package leetcode

func minimumRefill(plants []int, capacityA int, capacityB int) int {
	a, b := capacityA, capacityB
	i, j := 0, len(plants)-1
	ans := 0
	for i < j {
		if a < plants[i] {
			ans++
			a = capacityA
		}
		a -= plants[i]
		i++

		if b < plants[j] {
			ans++
			b = capacityB
		}
		b -= plants[j]
		j--
	}

	if i == j && max(a, b) < plants[i] {
		ans++
	}

	return ans
}
