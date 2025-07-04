package leetcode

func kthCharacterII(k int64, operations []int) byte {
	n := len(operations)
	if n == 0 {
		return 'a'
	}
	n--
	op := operations[n]
	operations = operations[:n]
	if n >= 63 || k <= 1<<n {
		return kthCharacterII(k, operations)
	}
	ans := kthCharacterII(k-1<<n, operations)
	return 'a' + (ans-'a'+byte(op))%26
}
