package main

import "testing"

func TestQuickSort(t *testing.T) {
	arrayToSort := []int{9, 3, 7, 4, 69, 420, 42}
	QuickSort(&arrayToSort)

	if slicesMatch(arrayToSort, []int{3, 4, 7, 9, 42, 69, 420}) == false {
		t.Fatalf("Array is not sorted: %v", arrayToSort)
	}
}

func slicesMatch(slice1 []int, slice2 []int) bool {
	if len(slice1) != len(slice2) {
		return false
	}

	for i, item := range slice1 {
		if item != slice2[i] {
			return false
		}
	}

	return true
}
