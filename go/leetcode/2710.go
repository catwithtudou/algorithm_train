package leetcode

func removeTrailingZeros(num string) string {
	i := len(num) - 1
	for ; i >= 0; i-- {
		if num[i] != '0' {
			break
		}
	}
	return num[:i+1]
	//return strings.TrimRight(num, "0")
}
