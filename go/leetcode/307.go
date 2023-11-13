package leetcode

import "math"

// 分块处理
type NumArray struct {
	nums, sums []int
	size       int
}

func NumArrayConstructor(nums []int) NumArray {
	n := len(nums)
	size := int(math.Sqrt(float64(n)))
	sums := make([]int, (n+size-1)/size) // 向上取整
	for i, num := range nums {
		sums[i/size] += num
	}
	return NumArray{nums, sums, size}
}

func (na *NumArray) Update(index int, val int) {
	na.sums[index/na.size] += val - na.nums[index]
	na.nums[index] = val
}

func (na *NumArray) SumRange(left int, right int) int {
	ans := 0
	size := na.size
	lChunk, rChunk := left/size, right/size
	if lChunk == rChunk {
		for i := left; i <= right; i++ {
			ans += na.nums[i]
		}
		return ans
	}
	for i := left; i < (lChunk+1)*size; i++ {
		ans += na.nums[i]
	}
	for i := lChunk + 1; i < rChunk; i++ {
		ans += na.sums[i]
	}
	for i := rChunk * size; i <= right; i++ {
		ans += na.nums[i]
	}
	return ans
}

/**
 * Your NumArray object will be instantiated and called as such:
 * obj := Constructor(nums);
 * obj.Update(index,val);
 * param_2 := obj.SumRange(left,right);
 */
