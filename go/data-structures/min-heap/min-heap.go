package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println("lets min some heaps")
}

type MinHeap struct {
	length int
	data   []int
}

func CreateMinHeap() *MinHeap {
	return &MinHeap{
		length: 0,
		data:   []int{},
	}
}

func (minHeap *MinHeap) Insert(value int) {
	minHeap.data = append(minHeap.data, value)
	minHeap.heapifyUp(minHeap.length)
	minHeap.length++
}

func (minHeap *MinHeap) Delete() int {
	if minHeap.length == 0 {
		return -1
	}

	minHeap.length--
	valueDeleted := minHeap.data[0]
	if minHeap.length == 0 {
		minHeap.data = []int{}

		return valueDeleted
	}

	valueAtEndOfTree := minHeap.data[minHeap.length]
	minHeap.data[0] = valueAtEndOfTree
	minHeap.data = minHeap.data[:minHeap.length]

	minHeap.heapifyDown(0)
	return valueDeleted
}

func (minHeap *MinHeap) heapifyDown(currentIndex int) {
	if currentIndex >= minHeap.length {
		return
	}

	leftChildIndex := getLeft(currentIndex)
	rightChildIndex := getRight(currentIndex)

	if leftChildIndex >= minHeap.length {
		return
	}

	leftChildValue := minHeap.data[leftChildIndex]

	var smallestChildIndex int
	var minValue float64
	if rightChildIndex >= minHeap.length {
		smallestChildIndex = leftChildIndex
		minValue = float64(leftChildValue)
	} else {
		rightChildValue := minHeap.data[rightChildIndex]

		minValue = math.Min(float64(leftChildValue), float64(rightChildValue))
		if leftChildValue < rightChildValue {
			smallestChildIndex = leftChildIndex
		} else {
			smallestChildIndex = rightChildIndex
		}

	}

	if minValue < float64(minHeap.data[currentIndex]) {
		minHeap.swap(currentIndex, smallestChildIndex)
		minHeap.heapifyDown(smallestChildIndex)
	}
}

func (minHeap *MinHeap) heapifyUp(currentIndex int) {
	if currentIndex == 0 {
		return
	}

	parentIndex := getParent(currentIndex)

	if minHeap.data[parentIndex] > minHeap.data[currentIndex] {
		minHeap.swap(parentIndex, currentIndex)
		minHeap.heapifyUp(parentIndex)
	}
}

func (minHeap *MinHeap) swap(index1, index2 int) {
	temp := minHeap.data[index1]
	minHeap.data[index1] = minHeap.data[index2]
	minHeap.data[index2] = temp
}

func getParent(currentIndex int) int {
	return (currentIndex - 1) / 2
}

func getLeft(currentIndex int) int {
	return (currentIndex * 2) + 1
}

func getRight(currentIndex int) int {
	return (currentIndex * 2) + 2
}

func (minHeap MinHeap) Print() {
	for _, value := range minHeap.data {
		fmt.Printf("%v, ", value)
	}
	fmt.Println()
}
