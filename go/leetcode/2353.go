package leetcode

import (
	"cmp"

	"github.com/emirpasic/gods/v2/trees/redblacktree"
)

type foodPair struct {
	food   string
	rating int
}

type FoodRatings struct {
	foodMap    map[string]foodPair
	cuisineMap map[string]*redblacktree.Tree[foodPair, struct{}]
}

func ConstructorFoodRatings(foods []string, cuisines []string, ratings []int) FoodRatings {
	foodMap := make(map[string]foodPair)
	cuisinesMap := make(map[string]*redblacktree.Tree[foodPair, struct{}])
	for i, food := range foods {
		cuisine, rating := cuisines[i], ratings[i]
		foodMap[food] = foodPair{cuisine, rating}
		if cuisinesMap[cuisine] == nil {
			cuisinesMap[cuisine] = redblacktree.NewWith[foodPair, struct{}](func(a, b foodPair) int {
				return cmp.Or(b.rating-a.rating, cmp.Compare(a.food, b.food))
			})
		}
		cuisinesMap[cuisine].Put(foodPair{food, rating}, struct{}{})
	}

	return FoodRatings{foodMap, cuisinesMap}
}

func (this *FoodRatings) ChangeRating(food string, newRating int) {
	p := this.foodMap[food]
	t := this.cuisineMap[p.food]
	t.Remove(foodPair{food, p.rating})
	t.Put(foodPair{food, newRating}, struct{}{})
	this.foodMap[food] = foodPair{p.food, newRating}
}

func (this *FoodRatings) HighestRated(cuisine string) string {
	return this.cuisineMap[cuisine].Left().Key.food
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * obj := Constructor(foods, cuisines, ratings);
 * obj.ChangeRating(food,newRating);
 * param_2 := obj.HighestRated(cuisine);
 */
