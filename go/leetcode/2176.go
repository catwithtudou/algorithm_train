package leetcode

var divisors [101][]int

func init() {
	for i := 1; i < 101; i++ {
		for j := i; j < 101; j += i {
			divisors[j] = append(divisors[j], i)
		}
	}
}

func countPairs2176(nums []int, k int) (ans int) {
	type pair struct{ v, d int }
	cnt := make(map[pair]int)
	for j, x := range nums {
		if j > 0 && x == nums[0] {
			ans++
		}
		k2 := k / gcd(k, j)
		ans += cnt[pair{x, k2}]
		for _, d := range divisors[j] {
			cnt[pair{x, d}]++
		}
	}

	return
}
