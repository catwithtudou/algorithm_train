package leetcode

type NumArray303 []int

func ConstructorNumArray303(nums []int) NumArray303 {
	s := make(NumArray303, len(nums)+1)
	for i, x := range nums {
		s[i+1] = s[i] + x
	}
	return s
}

func (n NumArray303) SumRange(left int, right int) int {
	return n[right+1] - n[left]
}

/**
 * Your NumArray object will be instantiated and called as such:
 * obj := Constructor(nums);
 * param_1 := obj.SumRange(left,right);
 */
