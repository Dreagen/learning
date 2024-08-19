package main

import (
	"fmt"

	"golang.org/x/exp/constraints"
)

func main() {
	fmt.Println("let's search some depths first")
}

type BinaryNode[T constraints.Ordered] struct {
	value  T
	parent *BinaryNode[T]
	left   *BinaryNode[T]
	right  *BinaryNode[T]
}

func DepthFirstSearch(head *BinaryNode[int], needle int) bool {
	if head == nil {
		return false
	}

	if head.value == needle {
		return true
	}

	if needle < head.value {
		return DepthFirstSearch(head.left, needle)
	}

	return DepthFirstSearch(head.right, needle)
}
