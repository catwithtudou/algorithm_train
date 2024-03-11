package leetcode

import "strings"

func capitalizeTitle(title string) string {
	a := strings.Split(title, " ")
	for i, s := range a {
		s = strings.ToLower(s)
		if len(s) > 2 {
			s = strings.Title(s)
		}
		a[i] = s
	}

	return strings.Join(a, " ")
}
