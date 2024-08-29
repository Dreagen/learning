package dijkstra

import (
	"adjacency-list"
	"math"
)

func Dijkstra(source, sink int, graph adjacencylist.WeightedAdjacencyList) []int {
	seen := make([]bool, len(graph.Value))
	dists := make([]float64, len(graph.Value))
	prev := make([]int, len(graph.Value))
	fill(&seen, false)
	fill(&dists, math.Inf(1))
	fill(&prev, -1)

	dists[source] = 0

	for hasUnvisited(seen, dists) {
		curr := getLowestUnvisited(seen, dists)
		seen[curr] = true

		adjs := graph.Value[curr]
		for i := 0; i < len(adjs); i++ {
			edge := adjs[i]
			if seen[edge.To] {
				continue
			}

			dist := dists[curr] + float64(edge.Weight)
			if dist < dists[edge.To] {
				dists[edge.To] = dist
				prev[edge.To] = curr
			}
		}
	}

	out := []int{}
	curr := sink
	for prev[curr] != -1 {
		out = append(out, curr)
		curr = prev[curr]
	}

	out = append(out, source)
	return reverse(out)
}

func hasUnvisited(seen []bool, dists []float64) bool {
	for i := 0; i < len(dists); i++ {
		if dists[i] < math.Inf(1) && seen[i] == false {
			return true
		}
	}

	return false
}

func getLowestUnvisited(seen []bool, dists []float64) int {
	index := -1
	lowestDistance := math.Inf(1)

	for i := 0; i < len(seen); i++ {
		if seen[i] {
			continue
		}

		if lowestDistance > dists[i] {
			lowestDistance = dists[i]
			index = i
		}
	}

	return index
}

func fill[T any](input *[]T, value T) {
	for i := range *input {
		(*input)[i] = value
	}
}

func reverse(input []int) []int {
	output := make([]int, len(input))
	for indexInInput, indexInOutput := len(input)-1, 0; indexInInput >= 0; indexInInput, indexInOutput = indexInInput-1, indexInOutput+1 {
		output[indexInOutput] = input[indexInInput]
	}

	return output
}
