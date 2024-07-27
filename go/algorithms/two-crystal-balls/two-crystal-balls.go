package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println("two crystal balls")
}

func TwoCrystalBalls(floors []bool) int {
	totalFloors := len(floors)
	jumpSize := math.Floor(math.Sqrt(float64(totalFloors)))

	low := 0
	for jumpSize < float64(totalFloors) {
		broke := floors[int(jumpSize)]
		if broke {
			for i := low; i < int(jumpSize); i++ {
				if floors[i] == true {
					return i
				}
			}
		} else {
			low = int(jumpSize)
			jumpSize = jumpSize * 2
		}
	}

	return -1
}
