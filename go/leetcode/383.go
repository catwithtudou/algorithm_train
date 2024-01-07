package leetcode

func canConstruct(ransomNote string, magazine string) bool {
	charMap := make(map[byte]int)
	for i := 0; i < len(magazine); i++ {
		char := magazine[i]
		charMap[char]++
	}
	for i := 0; i < len(ransomNote); i++ {
		char := ransomNote[i]
		if charMap[char] == 0 {
			return false
		}
		charMap[char]--
	}

	return true
}
