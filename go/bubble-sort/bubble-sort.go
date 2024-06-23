package main

import "fmt"

func main() {
	nums := []int{4, 6, 3, 1, 5, 6}
	sort(nums)
}

func sort(numbers []int) {
	length := len(numbers)
	for i := 0; i < length; length-- {
		for j := 0; j < len(numbers)-1; j++ {
			leftNumber := numbers[j]
			rightNumber := numbers[j+1]
			if leftNumber > rightNumber {
				numbers[j] = rightNumber
				numbers[j+1] = leftNumber
			}
		}
	}

	fmt.Println(numbers)
}
