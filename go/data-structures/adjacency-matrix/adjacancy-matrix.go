package main

import "fmt"

func main() {
	fmt.Println("Lets search some matrix's")
}

var WeightedAdjacencyMatrix [][]int

func BreadthFirstSearch(graph [][]int, source, needle int) []int {
	seen := make([]bool, len(graph))
	prev := make([]int, len(graph))
	fill(&seen, false)
	fill(&prev, -1)

	queue := []int{source}

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		if current == needle {
			break
		}

		adjs := graph[current]
		for i := 0; i < len(adjs); i++ {
			seen[current] = true

			if adjs[i] == 0 {
				continue
			}

			if seen[i] {
				continue
			}

			seen[i] = true
			prev[i] = current

			queue = append(queue, i)
		}
	}

	if prev[needle] == -1 {
		return []int{}
	}

	current := needle
	out := []int{}

	for prev[current] != -1 {
		out = append(out, current)
		current = prev[current]
	}

	out = reverse(out)
	sourceSlice := []int{source}

	return append(sourceSlice, out...)
}

func fill[T any](slice *[]T, value T) {
	for i := range *slice {
		(*slice)[i] = value
	}
}

func reverse[T any](slice []T) []T {
	reversedSlice := make([]T, len(slice))

	for i := len(slice) - 1; i >= 0; i-- {
		insertAt := len(slice) - 1 - i
		reversedSlice[insertAt] = (slice)[i]
	}

	return reversedSlice
}
