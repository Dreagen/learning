package main

import "testing"

func TestDepthFirstSearch(t *testing.T) {
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

	assertSearch(tree, t, 45, true)
	assertSearch(tree, t, 7, true)
	assertSearch(tree, t, 69, false)
}

func assertSearch(tree *BinaryNode[int], t *testing.T, needle int, expectedResult bool) {
	if DepthFirstSearch(tree, needle) != expectedResult {
		t.Fatalf("Return false when searching for %v, should have been %v", needle, expectedResult)
	}
}
