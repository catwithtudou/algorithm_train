package leetcode

type MyStack struct {
	queue []int
}

func ConstructorMyStack() MyStack {
	return MyStack{}
}

func (this *MyStack) Push(x int) {
	n := len(this.queue)
	this.queue = append(this.queue, x)
	for ; n > 0; n-- {
		this.queue = append(this.queue, this.queue[0])
		this.queue = this.queue[1:]
	}
}

func (this *MyStack) Pop() int {
	v := this.queue[0]
	this.queue = this.queue[1:]
	return v
}

func (this *MyStack) Top() int {
	return this.queue[0]
}

func (this *MyStack) Empty() bool {
	return len(this.queue) == 0
}

/**
 * Your MyStack object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Push(x);
 * param_2 := obj.Pop();
 * param_3 := obj.Top();
 * param_4 := obj.Empty();
 */
