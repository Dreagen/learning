package main

import "fmt"

func main() {
	input := []int{3, 2, 4}
	target := 6

	indicies := twoSum(input, target)
	fmt.Println(indicies)
}

func twoSum(nums []int, target int) []int {
	for index, element := range nums {
		result := inner(element, nums[index+1:], target, index)
		if result != nil {
			return result
		}
	}

	return nil
}

func inner(first int, nums []int, target int, starting int) []int {
	for index, element := range nums {
		if first+element == target {
			return []int{starting, starting + index + 1}
		}
	}
	return nil
}
