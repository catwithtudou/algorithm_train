package leetcode

func nextGreatestLetter(letters []byte, target byte) byte {
	n := len(letters)
	left, right := -1, n
	for left+1 < right {
		mid := left + (right-left)/2
		if letters[mid] > target {
			right = mid
		} else {
			left = mid
		}
	}
	if right == n {
		return letters[0]
	}

	return letters[right]
}
