package leetcode

import (
	"math/big"

	"golang.org/x/exp/slices"
)

func maxTotalRewardII(rewardValues []int) int {
	m := slices.Max(rewardValues)
	if slices.Contains(rewardValues, m-1) {
		return 2*m - 1
	}
	slices.Sort(rewardValues)
	rewardValues = slices.Compact(rewardValues)

	one := big.NewInt(1)
	f := big.NewInt(1)
	p := new(big.Int)
	for _, v := range rewardValues {
		mask := p.Sub(p.Lsh(one, uint(v)), one)
		f.Or(f, p.Lsh(p.And(f, mask), uint(v)))
	}
	return f.BitLen() - 1
}
