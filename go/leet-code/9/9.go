package main

import (
	"fmt"
	"strconv"
)

func main() {
	input := 121
	fmt.Println(isPalindrome(input))
}

func isPalindrome(x int) bool {
	inputAsString := strconv.Itoa(x)

	inputLength := len(inputAsString)

	digits := make([]int, inputLength)

	for index, char := range inputAsString {
		digits[index] = int(char)
	}

	middle := inputLength / 2
	if inputLength%2 != 0 {
		middle++
	}

	endIndex := inputLength - 1
	for startIndex := 0; startIndex < middle; startIndex++ {
		if digits[startIndex] != digits[endIndex] {
			return false
		}

		endIndex--
	}

	return true
}
