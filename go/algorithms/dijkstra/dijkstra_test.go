package dijkstra

import (
	"adjacency-list"
	"testing"
)

func TestDijkstra(t *testing.T) {
	expected := []int{0, 1, 4, 5, 6}
	actual := Dijkstra(0, 6, createGraph())

	if slicesMatch(actual, expected) == false {
		t.Fatalf("actual %v, did not match expected %v", actual, expected)
	}
}

func createGraph() adjacencylist.WeightedAdjacencyList {
	list := [][]adjacencylist.GraphEdge{}

	inner1 := []adjacencylist.GraphEdge{
		{To: 1, Weight: 3},
		{To: 2, Weight: 1},
	}
	inner2 := []adjacencylist.GraphEdge{
		{To: 0, Weight: 3},
		{To: 2, Weight: 4},
		{To: 4, Weight: 1},
	}
	inner3 := []adjacencylist.GraphEdge{
		{To: 1, Weight: 4},
		{To: 3, Weight: 7},
		{To: 0, Weight: 1},
	}
	inner4 := []adjacencylist.GraphEdge{
		{To: 2, Weight: 7},
		{To: 4, Weight: 5},
		{To: 6, Weight: 1},
	}
	inner5 := []adjacencylist.GraphEdge{
		{To: 1, Weight: 1},
		{To: 3, Weight: 5},
		{To: 5, Weight: 2},
	}
	inner6 := []adjacencylist.GraphEdge{
		{To: 6, Weight: 1},
		{To: 4, Weight: 2},
		{To: 2, Weight: 18},
	}
	inner7 := []adjacencylist.GraphEdge{
		{To: 3, Weight: 1},
		{To: 5, Weight: 1},
	}

	list = append(list, inner1)
	list = append(list, inner2)
	list = append(list, inner3)
	list = append(list, inner4)
	list = append(list, inner5)
	list = append(list, inner6)
	list = append(list, inner7)

	return adjacencylist.WeightedAdjacencyList{Value: list}
}

func slicesMatch(slice1, slice2 []int) bool {
	if len(slice1) != len(slice2) {
		return false
	}

	for i, value1 := range slice1 {
		value2 := slice2[i]

		if value1 != value2 {
			return false
		}
	}

	return true
}
