package leetcode

type FrequencyTracker struct {
	cnt  map[int]int
	freq map[int]int
}

func ConstructorFrequencyTracker() FrequencyTracker {
	return FrequencyTracker{map[int]int{}, map[int]int{}}
}

func (f *FrequencyTracker) update(number, delta int) {
	f.freq[f.cnt[number]]--
	f.cnt[number] += delta
	f.freq[f.cnt[number]]++
}

func (this *FrequencyTracker) Add(number int) {
	this.update(number, 1)
}

func (this *FrequencyTracker) DeleteOne(number int) {
	if this.cnt[number] > 0 {
		this.update(number, -1)
	}
}

func (this *FrequencyTracker) HasFrequency(frequency int) bool {
	return this.freq[frequency] > 0
}

/**
 * Your FrequencyTracker object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Add(number);
 * obj.DeleteOne(number);
 * param_3 := obj.HasFrequency(frequency);
 */
