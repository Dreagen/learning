package main

import (
	"fmt"
	"golang.org/x/exp/constraints"
)

func main() {
	fmt.Println("let's do a pre order binary tree search")
}

type BinaryNode[T constraints.Ordered] struct {
	value  T
	parent *BinaryNode[T]
	left   *BinaryNode[T]
	right  *BinaryNode[T]
}

func PreOrderSearch(head *BinaryNode[int]) []int {
	path := []int{}
	walk(head, &path)

	return path
}

func walk(current *BinaryNode[int], path *[]int) {
	if current == nil {
		return
	}

	*path = append(*path, current.value)
	walk(current.left, path)
	walk(current.right, path)

	return
}
