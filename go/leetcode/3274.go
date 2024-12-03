package leetcode

func checkTwoChessboards(coordinate1 string, coordinate2 string) bool {
	if (coordinate1[0]-'a')%2 == (coordinate2[0]-'a')%2 {
		return (coordinate1[1]-'1')%2 == (coordinate2[1]-'1')%2
	}

	return (coordinate1[1]-'1')%2 != (coordinate2[1]-'1')%2
}
