package main

import (
	"testing"
)

func TestBreadthFirstSearch(t *testing.T) {
	tree := &BinaryNode[int]{
		value: 20,
		right: &BinaryNode[int]{
			value: 50,
			right: &BinaryNode[int]{
				value: 100,
				right: nil,
				left:  nil,
			},
			left: &BinaryNode[int]{
				value: 30,
				right: &BinaryNode[int]{
					value: 45,
					right: nil,
					left:  nil,
				},
				left: &BinaryNode[int]{
					value: 29,
					right: nil,
					left:  nil,
				},
			},
		},
		left: &BinaryNode[int]{
			value: 10,
			right: &BinaryNode[int]{
				value: 15,
				right: nil,
				left:  nil,
			},
			left: &BinaryNode[int]{
				value: 5,
				right: &BinaryNode[int]{
					value: 7,
					right: nil,
					left:  nil,
				},
				left: nil,
			},
		},
	}

	found45 := BreadthFirstSearch(tree, 45)
	found7 := BreadthFirstSearch(tree, 7)
	found69 := BreadthFirstSearch(tree, 69)

	if found45 == false {
		t.Fatalf("Did not find 45 in tree and should have")
	}
	if found7 == false {
		t.Fatalf("Did not find 7 in tree and should have")
	}
	if found69 {
		t.Fatalf("Did find 69 in tree and should not have")
	}
}
