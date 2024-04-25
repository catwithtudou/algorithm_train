package leetcode

func distanceTraveled(mainTank, additionalTank int) (ans int) {
	for mainTank >= 5 {
		mainTank -= 5
		ans += 50
		if additionalTank > 0 {
			additionalTank--
			mainTank++
		}
	}
	return ans + mainTank*10
}
