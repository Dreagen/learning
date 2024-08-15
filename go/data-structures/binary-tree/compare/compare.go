package main

import (
	"fmt"
	"golang.org/x/exp/constraints"
)

func main() {
	fmt.Println("Lets compare some binary trees")
}

type BinaryNode[T constraints.Ordered] struct {
	value  T
	parent *BinaryNode[T]
	left   *BinaryNode[T]
	right  *BinaryNode[T]
}

func Compare(a, b *BinaryNode[int]) bool {
	if a == nil && b == nil {
		return true
	}

	if a == nil || b == nil {
		return false
	}

	if a.value != b.value {
		return false
	}

	return Compare(a.left, b.left) && Compare(a.right, b.right)
}
