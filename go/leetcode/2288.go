package leetcode

import (
	"fmt"
	"strconv"
	"strings"
)

func discountPrices(sentence string, discount int) string {
	d := 1 - float64(discount)/100
	a := strings.Split(sentence, " ")
	for i, w := range a {
		if len(w) > 1 && w[0] == '$' {
			price, err := strconv.Atoi(w[1:])
			if err == nil {
				a[i] = fmt.Sprintf("$%.2f", float64(price)*d)
			}
		}
	}
	return strings.Join(a, " ")
}
