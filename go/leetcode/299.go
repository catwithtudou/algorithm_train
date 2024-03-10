package leetcode

import "fmt"

func getHint(secret string, guess string) string {
	a, b := 0, 0
	var cntS, cntG [10]int
	for i := range secret {
		if secret[i] == guess[i] {
			a++
		} else {
			cntS[secret[i]-'0']++
			cntG[guess[i]-'0']++
		}
	}
	for i := 0; i < 10; i++ {
		b += min(cntS[i], cntG[i])
	}
	return fmt.Sprintf("%dA%dB", a, b)
}
