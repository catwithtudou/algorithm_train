package leetcode

type OrderedStream struct {
	List []string
	Ptr  int
}

func ConstructorOrderedStream(n int) OrderedStream {
	return OrderedStream{
		List: make([]string, n+1),
		Ptr:  1,
	}
}

func (this *OrderedStream) Insert(idKey int, value string) []string {
	this.List[idKey] = value
	start := this.Ptr
	for this.Ptr < len(this.List) && this.List[this.Ptr] != "" {
		this.Ptr++
	}
	return this.List[start:this.Ptr]
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * obj := Constructor(n);
 * param_1 := obj.Insert(idKey,value);
 */
