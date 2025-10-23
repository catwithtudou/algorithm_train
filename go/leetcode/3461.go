package leetcode

func hasSameDigits(s string) bool {
	n := len(s)
	arr := []byte(s)
	for i := 1; i <= n-2; i++ {
		for j := 0; j <= n-1-i; j++ {
			arr[j] = byte(((int(arr[j]-'0') + int(arr[j+1]-'0')) % 10) + '0')
		}
	}
	return arr[0] == arr[1]
}
