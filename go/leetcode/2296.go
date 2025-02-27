package leetcode

type TextEditor struct {
	left, right []byte
}

func ConstructorTextEditor() TextEditor {
	return TextEditor{}
}

func (this *TextEditor) AddText(text string) {
	this.left = append(this.left, text...)
	return
}

func (this *TextEditor) text() string {
	return string(this.left[max(0, len(this.left)-10):len(this.left)])
}

func (this *TextEditor) DeleteText(k int) int {
	k = min(len(this.left), k)
	this.left = this.left[:len(this.left)-k]
	return k
}

func (this *TextEditor) CursorLeft(k int) string {
	for k > 0 && len(this.right) > 0 {
		this.right = append(this.right, this.left[len(this.left)-1])
		this.left = this.left[:len(this.left)-1]
		k--
	}
	return this.text()
}

func (this *TextEditor) CursorRight(k int) string {
	for k > 0 && len(this.left) > 0 {
		this.left = append(this.left, this.right[len(this.right)-1])
		this.right = this.right[:len(this.right)-1]
		k--
	}
	return this.text()
}

/**
 * Your TextEditor object will be instantiated and called as such:
 * obj := Constructor();
 * obj.AddText(text);
 * param_2 := obj.DeleteText(k);
 * param_3 := obj.CursorLeft(k);
 * param_4 := obj.CursorRight(k);
 */
