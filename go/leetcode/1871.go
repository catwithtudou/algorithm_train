package leetcode

func canReach1871(s string, minJump int, maxJump int) bool {
	n := len(s)
	f := make([]int, n)
	sum := make([]int, n+1)
	f[0] = 1
	sum[1] = 1

	for j := 1; j < n; j++ {
		if j >= minJump && s[j] == '0' && sum[j-minJump+1] > sum[max(j-maxJump, 0)] {
			f[j] = 1
		}
		sum[j+1] = sum[j] + f[j]
	}

	return f[n-1] == 1
}
