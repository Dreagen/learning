package main

import (
	"fmt"
	"golang.org/x/exp/constraints"
)

func main() {
	fmt.Println("lets link some lists")
}

type Node[T constraints.Ordered] struct {
	value T
	prev  *Node[T]
	next  *Node[T]
}

type DoublyLinkedList[T constraints.Ordered] struct {
	length int
	head   *Node[T]
	tail   *Node[T]
}

func NewList[T constraints.Ordered]() *DoublyLinkedList[T] {
	return &DoublyLinkedList[T]{length: 0}
}

func (list *DoublyLinkedList[T]) Prepend(item T) {
	newNode := Node[T]{value: item}

	list.length++
	if list.head == nil {
		list.head = &newNode
		list.tail = &newNode
		return
	}

	list.head.prev = &newNode
	newNode.next = list.head
	list.head = &newNode
}

func (list *DoublyLinkedList[T]) InsertAt(item T, index int) {
	if index >= list.length-1 {
		list.Append(item)
		return
	}
	if index == 0 {
		list.Prepend(item)
		return
	}

	list.length++
	nodeToReplace := list.head
	for i := 1; i <= index; i++ {
		nodeToReplace = nodeToReplace.next
	}

	newNode := Node[T]{value: item}

	newNode.next = nodeToReplace
	newNode.prev = nodeToReplace.prev

	newNode.prev.next = &newNode
	nodeToReplace.prev = &newNode
}

func (list *DoublyLinkedList[T]) Append(item T) {
	newNode := Node[T]{value: item}

	list.length++
	if list.tail == nil {
		list.tail = &newNode
		list.head = &newNode
		return
	}

	newNode.prev = list.tail
	list.tail.next = &newNode
	list.tail = &newNode
}

func (list *DoublyLinkedList[T]) Remove(item T) {
	if list.head == nil {
		return
	}

	nodeToRemove := list.head
	for i := 0; i < list.length; i++ {
		if nodeToRemove.value == item {
			break
		}

		nodeToRemove = nodeToRemove.next
	}

	if nodeToRemove.value != item {
		return
	}

	list.length--

	if list.length == 0 {
		list.head = nil
		list.tail = nil
		return
	}

	if nodeToRemove.next != nil {
		nodeToRemove.next.prev = nodeToRemove.prev
	}
	if nodeToRemove.prev != nil {
		nodeToRemove.prev.next = nodeToRemove.next
	}

	if nodeToRemove == list.head {
		list.head = nodeToRemove.next
	}
	if nodeToRemove == list.tail {
		list.tail = nodeToRemove.prev
	}

	nodeToRemove.next = nil
	nodeToRemove.prev = nil
}

func (list *DoublyLinkedList[T]) Get(index int) T {
}

func (list *DoublyLinkedList[T]) RemoveAt(index int) {
}
