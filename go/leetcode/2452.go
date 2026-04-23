package leetcode

func twoEditWords(queries []string, dictionary []string) (ans []string) {

	for _, q := range queries {
	next:
		for _, s := range dictionary {
			cnt := 0
			for i := range s {
				if q[i] != s[i] {
					cnt++
					if cnt > 2 {
						continue next
					}
				}
			}
			ans = append(ans, q)
			break
		}
	}
	return ans
}
