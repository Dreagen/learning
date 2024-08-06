package main

import "fmt"

func main() {
	fmt.Println("let's do some quick sorting")
}

func QuickSort(arr *[]int) {
	qs(arr, 0, len(*arr)-1)
}

func qs(arr *[]int, low int, high int) {
	if low >= high {
		return
	}

	pivotIndex := parition(arr, low, high)

	qs(arr, low, pivotIndex-1)
	qs(arr, pivotIndex+1, high)
}

func parition(arr *[]int, low int, high int) int {
	pivot := (*arr)[high]

	index := low - 1

	for i := low; i < high; i++ {
		if (*arr)[i] <= pivot {
			index++
			tmp := (*arr)[i]
			(*arr)[i] = (*arr)[index]
			(*arr)[index] = tmp
		}
	}

	index++
	(*arr)[high] = (*arr)[index]
	(*arr)[index] = pivot

	return index
}
