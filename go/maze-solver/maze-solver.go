package main

import "fmt"

func main() {
	fmt.Println("maze solving time")
}

var dir = [][]int{
	{-1, 0},
	{1, 0},
	{0, -1},
	{0, 1},
}

func Walk(maze []string, wall rune, current Point, end Point, seen [][]bool, path *[]Point) bool {

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
		current = Point{x: current.x + dir[i][0], y: current.y + dir[i][1]}
		if Walk(maze, wall, current, end, seen, path) {
			return true
		}
	}

	*path = (*path)[:len(*path)-1]
	return false
}

func Solve(maze []string, wall rune, start, end Point) []Point {
	var seen [][]bool = make([][]bool, len(maze))
	var path []Point = make([]Point, 10000)

	for i := range seen {
		seen[i] = make([]bool, len(maze[0]))
	}

	Walk(maze, wall, start, end, make([][]bool, 0), &path)

	return path
}

type Point struct {
	x int
	y int
}
