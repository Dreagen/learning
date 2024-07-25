package main

import (
	"bytes"
	"flag"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strings"
)

func main() {
	command := "dotnet test"
	project := flag.String("project", "", "Project to run tests for e.g application or domain")
	flag.Parse()

	if *project == "" {
		runCommand(command)
		return
	}

	projectLocation, err := getTestProjectPath(*project)
	if err != nil {
		fmt.Println(err.Error())
		return
	}

	runCommand(command + " " + projectLocation)
}

func runCommand(command string) {

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
	var failureTestDescriptions []string
	failedLinesMap := make(map[string][]string)

	lines := strings.Split(stdoutStr, "\n")

	for _, line := range lines {
		if strings.Contains(line, "Passed!") && sliceContains(passedLines, line) == false {
			passedLines = append(passedLines, line)
		} else if strings.Contains(line, "Failed!") {
			if _, ok := failedLinesMap[line]; ok == false {
				failedLinesMap[line] = failureTestDescriptions
				failureTestDescriptions = []string{}
			}
		} else if hasTestFailureDescription(line) {
			failureTestDescriptions = append(failureTestDescriptions, line)
		}
	}

	fmt.Println()
	fmt.Println("Summary:")
	fmt.Println()

	for _, passedLine := range passedLines {
		fmt.Println(passedLine)
	}

	for key, failedTestDescriptions := range failedLinesMap {
		fmt.Println(key)
		for _, failedTestDescription := range failedTestDescriptions {
			fmt.Println(failedTestDescription)
		}
	}
}

func sliceContains(slice []string, value string) bool {
	for _, s := range slice {
		if s == value {
			return true
		}
	}

	return false
}

func hasTestFailureDescription(line string) bool {
	pattern := `\[\d+\s+ms\]\s*$`

	re := regexp.MustCompile(pattern)

	return re.MatchString(line)
}

func getTestProjectPath(search string) (string, error) {
	var foundPath string
	err := filepath.Walk(".", func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if info.IsDir() == false && strings.Contains(strings.ToLower(path), strings.ToLower(search)) && strings.HasSuffix(strings.ToLower(info.Name()), ".tests.csproj") {
			fmt.Printf("matched %s\n", path)
			foundPath = path
			return nil
		}

		return nil
	})

	return foundPath, err
}
