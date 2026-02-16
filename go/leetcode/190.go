package leetcode

const m0 = 0x55555555 // 01010101 ...
const m1 = 0x33333333 // 00110011 ...
const m2 = 0x0f0f0f0f // 00001111 ...
const m3 = 0x00ff00ff // 00000000111111110000000011111111
const m4 = 0x0000ffff // 00000000000000001111111111111111

func reverseBits(n int) int {
	n = n>>1&m0 | n&m0<<1
	n = n>>2&m1 | n&m1<<2
	n = n>>4&m2 | n&m2<<4
	n = n>>8&m3 | n&m3<<8
	return n>>16 | n&m4<<16
}
