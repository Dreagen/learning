package main

import (
	"fmt"

	"golang.org/x/exp/constraints"
)

func main() {
	fmt.Println("let's do a breadth first search")
}

type BinaryNode[T constraints.Ordered] struct {
	value  T
	parent *BinaryNode[T]
	left   *BinaryNode[T]
	right  *BinaryNode[T]
}

func BreadthFirstSearch(head *BinaryNode[int], needle int) bool {
	queue := []BinaryNode[int]{*head}

	for len(queue) > 0 {
		current := dequeue(&queue)
		fmt.Printf("Value at current %v\n", current.value)

		if current.value == needle {
			fmt.Printf("Found needle: %v with value: %v\n", needle, current.value)
			return true
		}

		if current.left != nil {
			queue = append(queue, *current.left)
		}

		if current.right != nil {
			queue = append(queue, *current.right)
		}
	}

	fmt.Printf("Did not find needle: %v\n", needle)
	return false
}

func dequeue(queue *[]BinaryNode[int]) BinaryNode[int] {
	firstItem := (*queue)[0]

	(*queue) = (*queue)[1:]

	return firstItem
}
