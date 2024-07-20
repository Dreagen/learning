package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"os/exec"
	"strings"
)

func main() {
	command := "dotnet test"

	var stdoutBuf bytes.Buffer

	stdoutMultiWriter := io.MultiWriter(os.Stdout, &stdoutBuf)

	// Use the `script` command to run `dotnet test` in a way that preserves colors
	scriptCommand := fmt.Sprintf("script -q -c '%s' /dev/null", command)
	cmd := exec.Command("sh", "-c", scriptCommand)

	cmd.Stdout = stdoutMultiWriter
	cmd.Stderr = os.Stderr

	if err := cmd.Start(); err != nil {
		fmt.Fprintln(os.Stderr, "Error starting Cmd:", err)
		return
	}

	if err := cmd.Wait(); err != nil {
		fmt.Fprintln(os.Stderr, "Error waiting for Cmd:", err)
		return
	}

	stdoutStr := stdoutBuf.String()

	var passedLines []string
	var failedLines []string
	lines := strings.Split(stdoutStr, "\n")

	for _, line := range lines {
		if strings.Contains(line, "Passed!") && SliceContains(passedLines, line) == false {
			passedLines = append(passedLines, line)
		} else if strings.Contains(line, "Failed!") && SliceContains(failedLines, line) == false {
			failedLines = append(failedLines, line)
		}
	}

	for _, passedLine := range passedLines {
		fmt.Println(passedLine)
	}

	for _, failedLine := range failedLines {
		fmt.Println(failedLine)
	}
}

func SliceContains(slice []string, value string) bool {
	for _, s := range slice {
		if s == value {
			return true
		}
	}

	return false
}
