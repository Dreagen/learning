package main

import "fmt"

func main() {
	fmt.Println("binary search")
}

func BinarySearch(haystack []int, needle int) bool {
	low := 0
	high := len(haystack)

	for low < high {
		mid := low + (high-low)/2
		value := haystack[mid]

		if needle == value {
			return true
		} else if needle > value {
			low = mid + 1
		} else {
			high = mid - 1
		}
	}

	return false
}
