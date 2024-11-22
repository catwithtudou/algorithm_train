package leetcode

func winningPlayerCount(n int, pick [][]int) int {
	cnts := make([][11]int, n)
	for _, p := range pick {
		cnts[p[0]][p[1]]++
	}
	ans := 0
	for i, cnt := range cnts {
		for _, c := range cnt {
			if c > i {
				ans++
				break
			}
		}
	}
	return ans
}
