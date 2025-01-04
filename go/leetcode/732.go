package leetcode

type MyCalendarThree struct {
	tm *segmentTree731
}

func ConstructorMyCalendarThree() MyCalendarThree {
	return MyCalendarThree{
		tm: newSegmentTree731(),
	}
}

func (this *MyCalendarThree) Book(startTime int, endTime int) int {
	this.tm.modify(startTime+1, endTime, 1, this.tm.root)
	return this.tm.query(1, 1e9, this.tm.root)
}
