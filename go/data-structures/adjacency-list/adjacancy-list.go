package main

func DepthFirstSearch(graph WeightedAdjacencyList, source, needle int) *[]int {
	seen := make([]bool, len(graph.value))
	path := &[]int{}
	fillSlice(&seen, false)

	result := walk(graph, source, needle, &seen, path)

	if result {
		return path
	}

	return nil
}

func walk(graph WeightedAdjacencyList, current, needle int, seen *[]bool, path *[]int) bool {
	if (*seen)[current] {
		return false
	}

	(*seen)[current] = true

	*path = append((*path), current)

	if current == needle {
		return true
	}

	list := graph.value[current]

	for i := 0; i < len(list); i++ {
		edge := list[i]

		if walk(graph, edge.to, needle, seen, path) {
			return true
		}
	}

	*path = (*path)[:len((*path))-1]

	return false

}

type GraphEdge struct {
	to     int
	weight int
}

func CreateGraphEdge(to, weight int) *GraphEdge {
	return &GraphEdge{to: to, weight: weight}
}

type WeightedAdjacencyList struct {
	value [][]GraphEdge
}

func fillSlice[T any](slice *[]T, value T) {
	for i := range *slice {
		(*slice)[i] = value
	}
}
