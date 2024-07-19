package main

import (
	"fmt"
	"log"
	"os/exec"
)

func main() {
	// Define the command and its arguments
	cmd := exec.Command("dotnet", "test")

	// Run the command and capture the output
	output, err := cmd.CombinedOutput()
	if err != nil {
		log.Fatalf("Command execution failed: %v", err)
	}

	// Print the output
	fmt.Printf("Output: %s\n", string(output))
}
