﻿package leetcode

import "math/bits"

func findProductsOfElements(queries [][]int64) []int {
	ans := make([]int, len(queries))
	for i, q := range queries {
		er := sumE(int(q[1]) + 1)
		el := sumE(int(q[0]))
		ans[i] = powMod(2, er-el, int(q[2]))
	}
	return ans
}

func sumE(k int) (res int) {
	var n, cnt1, sumI int
	for i := bits.Len(uint(k+1)) - 1; i > 0; i-- {
		c := cnt1<<i + i<<(i-1) // 新增的幂次个数
		if c <= k {
			k -= c
			res += sumI<<i + i*(i-1)/2<<(i-1)
			sumI += i   // 之前填的 1 的幂次之和
			cnt1++      // 之前填的 1 的个数
			n |= 1 << i // 填 1
		}
	}
	// 最低位单独计算
	if cnt1 <= k {
		k -= cnt1
		res += sumI
		n |= 1 // 最低位填 1
	}
	// 剩余的 k 个幂次，由 n 的低 k 个 1 补充
	for ; k > 0; k-- {
		res += bits.TrailingZeros(uint(n))
		n &= n - 1 // 去掉最低位的 1（置为 0）
	}
	return
}

func powMod(x, n, mod int) int {
	res := 1 % mod
	for ; n > 0; n /= 2 {
		if n%2 > 0 {
			res = res * x % mod
		}
		x = x * x % mod
	}
	return res
}
