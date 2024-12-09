package leetcode

func squareIsWhite(coordinates string) bool {
	return (coordinates[0]-'a'+1+coordinates[1]-'0')%2 == 1
}
