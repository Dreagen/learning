package main

import "testing"

func TestDepthFirstSearch(t *testing.T) {
	graph := createGraph()

	expected := []int{0, 1, 4, 5, 6}
	actual := DepthFirstSearch(graph, 0, 6)

	if slicesMatch(expected, (*actual)) == false {
		t.Fatalf("Result %v did not match expected %v", actual, expected)
	}
}

func createGraph() WeightedAdjacencyList {
	list := [][]GraphEdge{}

	inner1 := []GraphEdge{
		{to: 1, weight: 3},
		{to: 2, weight: 1},
	}
	inner2 := []GraphEdge{
		{to: 4, weight: 1},
	}
	inner3 := []GraphEdge{
		{to: 3, weight: 7},
	}
	inner4 := []GraphEdge{}
	inner5 := []GraphEdge{
		{to: 1, weight: 1},
		{to: 3, weight: 5},
		{to: 5, weight: 2},
	}
	inner6 := []GraphEdge{
		{to: 2, weight: 18},
		{to: 6, weight: 1},
	}
	inner7 := []GraphEdge{
		{to: 3, weight: 1},
	}

	list = append(list, inner1)
	list = append(list, inner2)
	list = append(list, inner3)
	list = append(list, inner4)
	list = append(list, inner5)
	list = append(list, inner6)
	list = append(list, inner7)

	return WeightedAdjacencyList{value: list}
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
