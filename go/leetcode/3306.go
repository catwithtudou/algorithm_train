package leetcode

func countOfSubstringsII(word string, k int) (ans int64) {
	const vowelMask = 1065233
	var cntVowel1, cntVowel2 ['u' - 'a' + 1]int
	sizeVowel1, sizeVowel2 := 0, 0
	cntConstant1, cntConstant2 := 0, 0
	left1, left2 := 0, 0

	for _, b := range word {

		b -= 'a'

		if vowelMask>>b&1 > 0 {
			if cntVowel1[b] == 0 {
				sizeVowel1++
			}
			cntVowel1[b]++
			if cntVowel2[b] == 0 {
				sizeVowel2++
			}
			cntVowel2[b]++
		} else {
			cntConstant1++
			cntConstant2++
		}

		for sizeVowel1 == 5 && cntConstant1 >= k {
			out := word[left1] - 'a'
			if vowelMask>>out&1 > 0 {
				cntVowel1[out]--
				if cntVowel1[out] == 0 {
					sizeVowel1--
				}
			} else {
				cntConstant1--
			}
			left1++
		}

		for sizeVowel2 == 5 && cntConstant2 > k {
			out := word[left2] - 'a'
			if vowelMask>>out&1 > 0 {
				cntVowel2[out]--
				if cntVowel2[out] == 0 {
					sizeVowel2--
				}
			} else {
				cntConstant2--
			}
			left2++
		}

		ans += int64(left1 - left2)
	}

	return ans
}
