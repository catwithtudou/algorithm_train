package leetcode

var diversionNum, diversionSum [100_001]int

func sumFourDivisorsInit() {
	for i := 1; i < 100_001; i++ {
		for j := i; j < 100_001; j += i {
			diversionNum[j]++
			diversionSum[j] += i
		}
	}
}

func sumFourDivisors(nums []int) (ans int) {
	for _, v := range nums {
		if diversionNum[v] == 4 {
			ans += diversionSum[v]
		}
	}

	return
}
