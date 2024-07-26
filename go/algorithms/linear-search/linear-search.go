package main

import "fmt"

func main() {
	input := []int{1, 2, 3, 4}
	fmt.Println(linearSearch(input, 4))
}

func linearSearch(haystack []int, needle int) bool {
	for _, v := range haystack {
		if v == needle {
			return true
		}
	}

	return false
}
