package main

import (
	"testing"
)

func TestMinHeap(t *testing.T) {
	heap := CreateMinHeap()

	if heap.length != 0 {
		t.Fatal("Length of new min heap is not 0 and should be")
	}

	heap.Insert(5)
	heap.Print()
	heap.Insert(3)
	heap.Print()
	heap.Insert(69)
	heap.Print()
	heap.Insert(420)
	heap.Print()
	heap.Insert(4)
	heap.Print()
	heap.Insert(1)
	heap.Print()
	heap.Insert(8)
	heap.Print()
	heap.Insert(7)
	heap.Print()

	expect(t, heap.length, 8)
	expect(t, heap.Delete(), 1)
	heap.Print()
	expect(t, heap.Delete(), 3)
	heap.Print()
	expect(t, heap.Delete(), 4)
	heap.Print()
	expect(t, heap.Delete(), 5)
	heap.Print()
	expect(t, heap.length, 4)
	expect(t, heap.Delete(), 7)
	heap.Print()
	expect(t, heap.Delete(), 8)
	heap.Print()
	expect(t, heap.Delete(), 69)
	heap.Print()
	expect(t, heap.Delete(), 420)
	heap.Print()
	expect(t, heap.length, 0)
}

func expect(t *testing.T, value1, value2 int) {
	if value1 != value2 {
		t.Fatalf("Expected %v to equal %v", value1, value2)
	}
}
