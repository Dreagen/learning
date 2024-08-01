package main

import "fmt"

func main() {
	fmt.Println("maze solving time")
}

func Solve(maze []string, wall string, start, end Point) []Point {
	return []Point{}
}

type Point struct {
	x int
	y int
}
