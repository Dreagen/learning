package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println("maze solving time")
}

var dir = [][]int{
	{-1, 0},
	{1, 0},
	{0, -1},
	{0, 1},
}

func walk(maze []string, wall rune, current Point, end Point, seen [][]bool, path *[]Point, iterations *int) bool {

	*iterations++
	if current.x < 0 || current.x >= len(maze[0]) || current.y < 0 || current.y >= len(maze) {
		return false
	}

	if maze[current.y][current.x] == byte(wall) {
		return false
	}

	if seen[current.y][current.x] {
		return false
	}

	if current.x == end.x && current.y == end.y {
		*path = append(*path, current)
		return true
	}

	seen[current.y][current.x] = true
	*path = append(*path, current)

	for i := 0; i < len(dir); i++ {
		if walk(maze, wall, Point{x: current.x + dir[i][0], y: current.y + dir[i][1]}, end, seen, path, iterations) {
			return true
		}
	}

	*path = (*path)[:len(*path)-1]
	return false
}

func Solve(maze []string, wall rune, start, end Point) []Point {
	seen := make([][]bool, len(maze))
	var path []Point

	for i := range seen {
		seen[i] = make([]bool, len(maze[0]))
	}

	iterations := 0
	walk(maze, wall, start, end, seen, &path, &iterations)

	printPath(maze, path)
	fmt.Printf("Number of iterations %d\n", iterations)
	return path
}

func printPath(maze []string, path []Point) {
	var result []string = make([]string, len(maze))
	for y := range maze {
		pointsAtY := findPointsAtY(path, y)

		var builder strings.Builder
		for x := range maze[0] {
			if pointsContainX(pointsAtY, x) {
				builder.WriteString("x")
			} else {
				builder.WriteString("-")
			}
		}

		result[y] = builder.String()
	}

	fmt.Println()
	for _, item := range result {
		fmt.Println(item)
	}
	fmt.Println()
}

func findPointsAtY(path []Point, y int) []Point {
	var points []Point
	for _, point := range path {
		if point.y == y {
			points = append(points, point)
		}
	}

	return points
}

func pointsContainX(points []Point, x int) bool {
	for _, point := range points {
		if point.x == x {
			return true
		}
	}

	return false
}

type Point struct {
	x int
	y int
}
