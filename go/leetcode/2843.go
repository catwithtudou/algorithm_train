package leetcode

func countSymmetricIntegers(low int, high int) (ans int) {

	for i := low; i <= high; i++ {
		if i > 10 && i < 100 {
			if i%10 == i/10 {
				ans++
			}
		}
		if i > 1000 && i < 10000 {
			if (i%10 + (i/10)%10) == ((i/100)%10 + i/1000) {
				ans++
			}
		}
	}

	return
}
