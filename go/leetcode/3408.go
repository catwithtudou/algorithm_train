package leetcode

import (
	"cmp"
	"container/heap"
)

type TaskManagerPair struct{ priority, userId int }

type TaskManager struct {
	mp map[int]TaskManagerPair // taskId -> (priority, userId)
	h  *TaskManagerHp          // (priority, taskId, userId)
}

func TaskManagerConstructor(tasks [][]int) TaskManager {
	n := len(tasks)
	mp := make(map[int]TaskManagerPair, n) // 预分配空间
	h := make(TaskManagerHp, n)
	for i, t := range tasks {
		userId, taskId, priority := t[0], t[1], t[2]
		mp[taskId] = TaskManagerPair{priority, userId}
		h[i] = TaskManagerTuple{priority, taskId, userId}

	}
	heap.Init(&h)
	return TaskManager{mp, &h}
}

func (t *TaskManager) Add(userId, taskId, priority int) {
	t.mp[taskId] = TaskManagerPair{priority, userId}
	heap.Push(t.h, TaskManagerTuple{priority, taskId, userId})
}

func (t *TaskManager) Edit(taskId, newPriority int) {
	// 懒修改
	t.Add(t.mp[taskId].userId, taskId, newPriority)
}

func (t *TaskManager) Rmv(taskId int) {
	// 懒删除
	t.mp[taskId] = TaskManagerPair{-1, -1}
}

func (t *TaskManager) ExecTop() int {
	for t.h.Len() > 0 {
		top := heap.Pop(t.h).(TaskManagerTuple)
		priority, taskId, userId := top.priority, top.taskId, top.userId
		if t.mp[taskId] == (TaskManagerPair{priority, userId}) {
			t.Rmv(taskId)
			return userId
		}
	}
	return -1
}

type TaskManagerTuple struct{ priority, taskId, userId int }
type TaskManagerHp []TaskManagerTuple

func (h TaskManagerHp) Len() int { return len(h) }
func (h TaskManagerHp) Less(i, j int) bool {
	return cmp.Or(h[i].priority-h[j].priority, h[i].taskId-h[j].taskId) > 0
}
func (h TaskManagerHp) Swap(i, j int) { h[i], h[j] = h[j], h[i] }
func (h *TaskManagerHp) Push(v any)   { *h = append(*h, v.(TaskManagerTuple)) }
func (h *TaskManagerHp) Pop() any     { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }
