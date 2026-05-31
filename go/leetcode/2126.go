package leetcode

import "slices"

func asteroidsDestroyed(mass int, asteroids []int) bool {
	slices.Sort(asteroids)

	for _, a := range asteroids {
		if mass < a {
			return false
		}
		mass += a
	}

	return true
}
