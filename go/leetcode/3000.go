package leetcode

func areaOfMaxDiagonal(dimensions [][]int) int {
	maxIdx := 0
	preNum := dimensions[0][0]*dimensions[0][0] + dimensions[0][1]*dimensions[0][1]
	for i := 1; i < len(dimensions); i++ {
		curNum := dimensions[i][0]*dimensions[i][0] + dimensions[i][1]*dimensions[i][1]
		if curNum > preNum {
			maxIdx = i
			preNum = curNum
		}
		if curNum == preNum {
			if dimensions[i][0]*dimensions[i][1] > dimensions[maxIdx][0]*dimensions[maxIdx][1] {
				maxIdx = i
			}
		}
	}
	return dimensions[maxIdx][0] * dimensions[maxIdx][1]
}
