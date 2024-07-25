package main

import "fmt"

func main() {
	nums := []int{4, 6, 3, 1, 5, 6}
	sort2(nums)

	fmt.Println(nums)
}

func sort(nums []int) {
	length := len(nums)
	for i := 0; i < length; length-- {
		for j := 0; j < len(nums)-1; j++ {
			leftNumber := nums[j]
			rightNumber := nums[j+1]
			if leftNumber > rightNumber {
				nums[j] = rightNumber
				nums[j+1] = leftNumber
			}
		}
	}
}

func sort2(nums []int) {
	for i := 0; i < len(nums); i++ {
		for j := 0; j < len(nums)-1-i; j++ {
			if nums[j] > nums[j+1] {
				swap(nums, j, j+1)
			}
		}
	}
}

func swap(array []int, index1 int, index2 int) {
	temp := array[index1]
	array[index1] = array[index2]
	array[index2] = temp
}
