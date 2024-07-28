package main

import (
	"errors"
	"fmt"
)

func main() {
	fmt.Println("Queue")
}

type QueueNode[T any] struct {
	value T
	next  *QueueNode[T]
}

type Queue[T any] struct {
	length int
	head   *QueueNode[T]
	tail   *QueueNode[T]
}

func NewQueue[T any]() *Queue[T] {
	return &Queue[T]{length: 0}
}

func (queue *Queue[T]) Enqueue(value T) {
	node := QueueNode[T]{value: value}
	queue.tail.next = &node
	queue.tail = &node

	if queue.head == nil {
		queue.head = &node
	}

	queue.length++
}

func (queue *Queue[T]) Dequeue() (T, error) {
	if queue.head == nil {
		var zeroValue T
		return zeroValue, errors.New("queue is empty")
	}

	value := queue.head.value
	queue.head = queue.head.next

	if queue.head == nil {
		queue.tail = nil
	}

	queue.length--

	return value, nil
}

func (queue *Queue[T]) Peek() (T, error) {
	if queue.head == nil {
		var zeroValue T
		return zeroValue, errors.New("queue is empty")
	}

	return queue.head.value, nil
}
