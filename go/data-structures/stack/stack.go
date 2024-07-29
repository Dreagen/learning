package main

import (
	"errors"
	"fmt"
)

func main() {
	fmt.Println("Stack implementation")
}

type StackNode[T any] struct {
	value    T
	previous *StackNode[T]
}

type Stack[T any] struct {
	length int
	head   *StackNode[T]
}

func NewStack[T any]() *Stack[T] {
	return &Stack[T]{length: 0}
}

func (stack *Stack[T]) push(value T) {
	node := &StackNode[T]{value: value}

	stack.length++

	if stack.head == nil {
		stack.head = node
		return
	}

	node.previous = stack.head
	stack.head = node
}

func (stack *Stack[T]) pop() (T, error) {
	if stack.length == 0 {
		var zeroValue T
		return zeroValue, errors.New("Can't pop from an empty stack")
	}

	returnValue := stack.head.value

	if stack.head.previous != nil {
		stack.head = stack.head.previous
	} else {
		stack.head = nil
	}

	stack.length--

	return returnValue, nil
}

func (stack *Stack[T]) peek() (T, error) {
	if stack.length == 0 {
		var zeroValue T
		return zeroValue, errors.New("Can't peak an empty stack")
	}

	return stack.head.value, nil
}
