package leetcode

type BrowserHistory struct {
	stack []string
	index int
}

func ConstructorBrowserHistory(homepage string) BrowserHistory {
	return BrowserHistory{
		stack: []string{homepage},
		index: 0,
	}
}

func (this *BrowserHistory) Visit(url string) {
	this.stack = this.stack[:this.index+1]
	this.stack = append(this.stack, url)
	this.index++
}

func (this *BrowserHistory) Back(steps int) string {
	if this.index-steps < 0 {
		this.index = 0
		return this.stack[this.index]
	}
	this.index -= steps
	return this.stack[this.index]
}

func (this *BrowserHistory) Forward(steps int) string {
	if this.index+steps >= len(this.stack) {
		this.index = len(this.stack) - 1
		return this.stack[this.index]
	}
	this.index += steps
	return this.stack[this.index]
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * obj := Constructor(homepage);
 * obj.Visit(url);
 * param_2 := obj.Back(steps);
 * param_3 := obj.Forward(steps);
 */
