package leetcode

func destCity(paths [][]string) string {

	pathMap := make(map[string]bool, len(paths))
	for _, path := range paths {
		pathMap[path[0]] = true
	}

	for _, path := range paths {
		if _, ok := pathMap[path[1]]; !ok {
			return path[1]
		}
	}

	return ""
}
