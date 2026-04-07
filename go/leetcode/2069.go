package leetcode

type Robot struct {
	w, h, step int
}

func RobotConstructor(width int, height int) Robot {
	return Robot{width, height, 0}
}

func (this *Robot) Step(num int) {
	this.step = (this.step+num-1)%((this.w+this.h-2)*2) + 1
}

func (this *Robot) GetState() (x, y int, dir string) {
	w, h, step := this.w, this.h, this.step
	switch {
	case step < w:
		return step, 0, "East"
	case step < h+w:
		return w, step - w + 1, "North"
	case step < w*2+h-2:
		return w*2 + h - step - 3, h - 1, "West"
	}
	return 0, (w+h)*2 - step - 4, "South"
}

func (this *Robot) GetPos() []int {
	x, y, _ := this.GetState()
	return []int{x, y}
}

func (this *Robot) GetDir() string {
	_, _, dir := this.GetState()
	return dir
}

/**
 * Your Robot object will be instantiated and called as such:
 * obj := Constructor(width, height);
 * obj.Step(num);
 * param_2 := obj.GetPos();
 * param_3 := obj.GetDir();
 */
