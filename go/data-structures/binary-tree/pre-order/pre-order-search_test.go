package main

import (
	"testing"
)

func TestPreOrderSearch(t *testing.T) {
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

	expected := []int{
		20,
		10,
		5,
		7,
		15,
		50,
		30,
		29,
		45,
		100,
	}
	actual := PreOrderSearch(tree)

	if compareSlices(expected, actual) == false {
		t.Fatalf("Expected %v, but was %v", expected, actual)
	}
}

func compareSlices(slice1, slice2 []int) bool {
	if len(slice1) != len(slice2) {
		return false
	}

	for i, v := range slice1 {
		if v != slice2[i] {
			return false
		}
	}

	return true
}
