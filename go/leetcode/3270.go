package leetcode

func generateKey(num1 int, num2 int, num3 int) (ans int) {
	for i := 1; num1 > 0 && num2 > 0 && num3 > 0; i *= 10 {
		res := min(num1%10, num2%10)
		res = min(res, num3%10)
		ans += res * i
		num1 /= 10
		num2 /= 10
		num3 /= 10
	}
	return
}
