package main

import "fmt"

func main() {
	input := []int{3, 2, 4}
	target := 6

	indicies := twoSum(input, target)
	fmt.Println(indicies)
}

func twoSum(nums []int, target int) []int {
	m := make(map[int]int)
	for index, element := range nums {
		need := target - element
		if secondIndex, found := m[need]; found {
			return []int{index, secondIndex}
		}

		m[element] = index
	}

	return nil
}
