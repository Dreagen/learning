package main

import "fmt"

func main() {
	var a [5]int
	fmt.Println("emt:", a)

	a[4] = 100
	fmt.Println("set:", a)
	fmt.Println("get:", a[4])
	fmt.Println("length", len(a))

	b := [5]int{1, 2, 3, 4, 5}
	fmt.Println("dc1", b)

	c := [...]int{1, 2, 3, 4, 5}
	fmt.Println("dc2", c)
}
