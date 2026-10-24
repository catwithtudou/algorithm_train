package leetcode

var daysPreSum = []int{0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365}

func countDaysTogether(arriveAlice string, leaveAlice string, arriveBob string, leaveBob string) int {
	calDay := func(s string) int {
		month := (s[0]-'0')*10 + (s[1] - '0')
		day := (s[3]-'0')*10 + (s[4] - '0')
		return daysPreSum[month-1] + int(day)
	}

	left := calDay(maxString(arriveAlice, arriveBob))
	right := calDay(minString(leaveAlice, leaveBob))

	return max(right-left+1, 0)
}

func maxString(a string, b string) string {
	if a > b {
		return a
	}
	return b
}

func minString(a string, b string) string {
	if a > b {
		return b
	}
	return a
}
