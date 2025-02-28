use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
struct Pair {
    rating: i32,
    food: String,
}

// 为Pair实现比较特性以用于优先队列
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rating.cmp(&other.rating) {
            Ordering::Equal => other.food.cmp(&self.food),
            self_ordering => self_ordering,
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
pub struct FoodRatings {
    food_map: HashMap<String, (i32, String)>, // 食物 -> (评分, 菜系)
    cuisine_map: HashMap<String, BinaryHeap<Pair>>, // 菜系 -> 食物及其评分的优先队列
}

#[allow(dead_code)]
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_map = HashMap::new();
        let mut cuisine_map = HashMap::new();

        for i in 0..foods.len() {
            let food = &foods[i];
            let cuisine = &cuisines[i];
            let rating = ratings[i];

            food_map.insert(food.clone(), (rating, cuisine.clone()));

            let heap = cuisine_map
                .entry(cuisine.clone())
                .or_insert_with(BinaryHeap::new);

            heap.push(Pair {
                rating,
                food: food.clone(),
            })
        }

        FoodRatings {
            food_map,
            cuisine_map,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        if let Some((rating, cuisine)) = self.food_map.get_mut(&food) {
            *rating = new_rating;

            if let Some(heap) = self.cuisine_map.get_mut(cuisine) {
                heap.push(Pair {
                    rating: new_rating,
                    food: food.clone(),
                });
            }
        }
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        if let Some(heap) = self.cuisine_map.get_mut(&cuisine) {
            while let Some(top) = heap.peek() {
                if let Some((cur_rating, _)) = self.food_map.get(&top.food) {
                    if *cur_rating == top.rating {
                        return top.food.clone();
                    }
                }
                heap.pop();
            }
        }

        String::new()
    }
}
