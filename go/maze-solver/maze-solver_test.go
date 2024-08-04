package main

import (
	"fmt"
	"testing"
)

func TestMaze1(t *testing.T) {

	maze := []string{
		"xxxxxxxxxx x",
		"x        x x",
		"x        x x",
		"x xxxxxxxx x",
		"x          x",
		"x xxxxxxxxxx",
	}

	expectedPath := []Point{
		{x: 10, y: 0},
		{x: 10, y: 1},
		{x: 10, y: 2},
		{x: 10, y: 3},
		{x: 10, y: 4},
		{x: 9, y: 4},
		{x: 8, y: 4},
		{x: 7, y: 4},
		{x: 6, y: 4},
		{x: 5, y: 4},
		{x: 4, y: 4},
		{x: 3, y: 4},
		{x: 2, y: 4},
		{x: 1, y: 4},
		{x: 1, y: 5},
	}

	actualPath := Solve(maze, 'x', Point{x: 10, y: 0}, Point{x: 1, y: 5})

	if pathsMatch(actualPath, expectedPath) == false {
		t.Fatal("Paths did not match")
	}
}

func TestMaze2(t *testing.T) {

	maze := []string{
		"xxx xxxx xxx",
		"xxx   xx xxx",
		"x   x      x",
		"xxx xxxxxx x",
		"x          x",
		"x xxxxxxxx x",
	}

	expectedPath := []Point{
		{x: 3, y: 0},
		{x: 3, y: 1},
		{x: 4, y: 1},
		{x: 5, y: 1},
		{x: 5, y: 2},
		{x: 6, y: 2},
		{x: 7, y: 2},
		{x: 8, y: 2},
		{x: 9, y: 2},
		{x: 10, y: 2},
		{x: 10, y: 3},
		{x: 10, y: 4},
		{x: 9, y: 4},
		{x: 8, y: 4},
		{x: 7, y: 4},
		{x: 6, y: 4},
		{x: 5, y: 4},
		{x: 4, y: 4},
		{x: 3, y: 4},
		{x: 2, y: 4},
		{x: 1, y: 4},
		{x: 1, y: 5},
	}

	actualPath := Solve(maze, 'x', Point{x: 3, y: 0}, Point{x: 1, y: 5})

	if pathsMatch(actualPath, expectedPath) == false {
		t.Fatal("Paths did not match")
	}
}

func pathsMatch(actualPath []Point, expectedPath []Point) bool {
	if len(actualPath) != len(expectedPath) {
		fmt.Printf("actual and expected do now match in length, actual: %d, expected %d", len(actualPath), len(expectedPath))
		return false
	}

	for i, actualPoint := range actualPath {
		expectedPoint := expectedPath[i]
		if actualPoint.x != expectedPoint.x && actualPoint.y != expectedPoint.y {
			return false
		}
	}

	return true
}
