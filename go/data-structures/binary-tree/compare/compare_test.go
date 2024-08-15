package main

import (
	"testing"
)

func TestCompare(t *testing.T) {
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

	tree1 := &BinaryNode[int]{
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
					value: 30,
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

	if Compare(tree, tree) == false {
		t.Fatal("tree did not match and should have")
	}

	if Compare(tree, tree1) {
		t.Fatal("tree matched and should not have")
	}
}
