package leetcode

import (
	"maps"
	"slices"
)

func braceExpansionII(expression string) []string {
	type set map[string]struct{}
	i := 0

	var dfs func() set

	dfs = func() set {
		res := set{}
		cur := set{"": {}}

		for i < len(expression) {
			ch := expression[i]
			i++

			if ch == '}' {
				break
			}

			if ch == ',' {
				maps.Copy(res, cur)
				cur = set{"": {}}
			} else if ch == '{' {
				subRes := dfs()
				newSet := make(set, len(cur)*len(subRes))
				for s := range cur {
					for t := range subRes {
						newSet[s+t] = struct{}{}
					}
				}
				cur = newSet
			} else {
				newSet := make(set, len(cur))
				t := string(ch)
				for s := range cur {
					newSet[s+t] = struct{}{}
				}
				cur = newSet
			}
		}

		maps.Copy(res, cur)
		return res
	}

	return slices.Sorted(maps.Keys(dfs()))
}
