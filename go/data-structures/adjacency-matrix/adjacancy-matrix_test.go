package main

import "testing"

func TestBfsAdjMatrix(t *testing.T) {

	matrix2 := [][]int{
		{0, 3, 1, 0, 0, 0, 0}, // 0
		{0, 0, 0, 0, 1, 0, 0},
		{0, 0, 7, 0, 0, 0, 0},
		{0, 0, 0, 0, 0, 0, 0},
		{0, 1, 0, 5, 0, 2, 0},
		{0, 0, 18, 0, 0, 0, 1},
		{0, 0, 0, 1, 0, 0, 1},
	}

	expected := []int{0, 1, 4, 5, 6}
	actual := BreadthFirstSearch(matrix2, 0, 6)

	if compareSlices(actual, expected) == false {
		t.Fatalf("actual %v did not match expected %v", actual, expected)
	}
}

func compareSlices(slice1, slice2 []int) bool {
	if len(slice1) != len(slice2) {
		return false
	}
	for i, slice1Value := range slice1 {
		slice2Value := slice2[i]

		if slice1Value != slice2Value {
			return false
		}
	}

	return true
}
