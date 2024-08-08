package main

import "testing"

func TestLinkedList(t *testing.T) {
	linkedList := NewList[int]()

	linkedList.Append(5)
	linkedList.Append(7)
	linkedList.Append(9)

	expect(t, linkedList.Get(2), 9)
	linkedList.RemoveAt(1)
	expect(t, linkedList.length, 2)

	linkedList.Append(11)
	linkedList.RemoveAt(1)
	linkedList.Remove(9)
	linkedList.RemoveAt(0)
	linkedList.RemoveAt(0)
	expect(t, linkedList.length, 0)

	linkedList.Prepend(5)
	linkedList.Prepend(7)
	linkedList.Prepend(9)

	expect(t, linkedList.Get(2), 5)
	expect(t, linkedList.Get(0), 9)
	linkedList.Remove(9)
	expect(t, linkedList.length, 2)
	expect(t, linkedList.Get(0), 7)
}

func expect(t *testing.T, item1, item2 int) {
	if item1 != item2 {
		t.Fatalf("%v did not match expected %v", item1, item2)
	}
}
