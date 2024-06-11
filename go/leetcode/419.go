package leetcode

func countBattleships(board [][]byte) int {
	ans := 0
	for i, row := range board {
		for j, c := range row {
			if c == 'X' && (j == 0 || row[j-1] != 'X') && (i == 0 || board[i-1][j] != 'X') {
				ans++
			}
		}
	}

	return ans
}
