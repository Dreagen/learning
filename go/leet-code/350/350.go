package main

import "fmt"

func main() {
	input1 := []int{1, 2, 2, 1}
	input2 := []int{2}
	fmt.Println(intersect(input1, input2))
}

func intersect(nums1 []int, nums2 []int) []int {
	var output []int

	for _, element := range nums1 {
		foundAtIndex := -1
		for index, element2 := range nums2 {
			if element == element2 {
				output = append(output, element)
				foundAtIndex = index
				break
			}
		}
		if foundAtIndex != -1 {
			nums2 = removeFromSlice(nums2, foundAtIndex)
		}
	}

	return output
}

func removeFromSlice(slice []int, index int) []int {
	return append(slice[:index], slice[index+1:]...)
}
