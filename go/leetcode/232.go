package leetcode

type MyQueue struct {
	inS, outS []int
}

func ConstructorMyQueue() MyQueue {
	return MyQueue{}
}

func (this *MyQueue) Push(x int) {
	this.inS = append(this.inS, x)
}

func (this *MyQueue) inToOut() {
	for len(this.inS) > 0 {
		this.outS = append(this.outS, this.inS[len(this.inS)-1])
		this.inS = this.inS[:len(this.inS)-1]
	}
}

func (this *MyQueue) Pop() int {
	if len(this.outS) == 0 {
		this.inToOut()
	}
	ans := this.outS[len(this.outS)-1]
	this.outS = this.outS[:len(this.outS)-1]
	return ans
}

func (this *MyQueue) Peek() int {
	if len(this.outS) == 0 {
		this.inToOut()
	}
	return this.outS[len(this.outS)-1]
}

func (this *MyQueue) Empty() bool {
	return len(this.inS) == 0 && len(this.outS) == 0
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Push(x);
 * param_2 := obj.Pop();
 * param_3 := obj.Peek();
 * param_4 := obj.Empty();
 */
