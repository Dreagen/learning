package main

import (
	"testing"
)

var input = []int{1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420}

func Test69(t *testing.T) {
	testInput(t, input, 69, true)
}

func Test1336(t *testing.T) {
	testInput(t, input, 1336, false)
}

func Test69420(t *testing.T) {
	testInput(t, input, 69420, true)
}

func Test69421(t *testing.T) {
	testInput(t, input, 69421, false)
}

func Test1(t *testing.T) {
	testInput(t, input, 1, true)
}

func Test0(t *testing.T) {
	testInput(t, input, 0, false)
}

func testInput(t *testing.T, input []int, search int, expected bool) {
	if BinarySearch(input, search) != expected {
		t.Fatalf("Value %d was not found and should have been", search)
	}
}
