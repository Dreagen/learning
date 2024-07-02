package main

import (
	"fmt"
	"log"

	"example.com/greetings"
)

func main() {
	log.SetPrefix("greetings: ")
	log.SetFlags(0)

	message, error := greetings.Hello("Nick")

	if error != nil {
		log.Fatal(error)
	}

	fmt.Println(message)
}
