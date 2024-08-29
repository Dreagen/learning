package adjacencylist

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
		{To: 1, Weight: 3},
		{To: 2, Weight: 1},
	}
	inner2 := []GraphEdge{
		{To: 4, Weight: 1},
	}
	inner3 := []GraphEdge{
		{To: 3, Weight: 7},
	}
	inner4 := []GraphEdge{}
	inner5 := []GraphEdge{
		{To: 1, Weight: 1},
		{To: 3, Weight: 5},
		{To: 5, Weight: 2},
	}
	inner6 := []GraphEdge{
		{To: 2, Weight: 18},
		{To: 6, Weight: 1},
	}
	inner7 := []GraphEdge{
		{To: 3, Weight: 1},
	}

	list = append(list, inner1)
	list = append(list, inner2)
	list = append(list, inner3)
	list = append(list, inner4)
	list = append(list, inner5)
	list = append(list, inner6)
	list = append(list, inner7)

	return WeightedAdjacencyList{Value: list}
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
