package main

import (
	"bufio"
	"bytes"
	"flag"
	"fmt"
	"io"
	"os"
)

func main() {
	bytesFlag := flag.Bool("c", false, "print the byte counts")
	newLineFlag := flag.Bool("l", false, "print the newline counts")
	wordFlag := flag.Bool("w", false, "print the word counts")
	characterFlag := flag.Bool("m", false, "print the character counts")

	flag.Parse()
	args := flag.Args()

	var filePath string
	var file *os.File
	var err error
	var reader io.ReadSeeker

	if len(args) < 1 {
		file = os.Stdin
		content, err := io.ReadAll(os.Stdin)
		if err != nil {
			fmt.Printf("Could not read file from stdin: %v\n", err.Error())
		}

		reader = bytes.NewReader(content)
	} else {
		filePath = args[0]
		file, err = os.Open(filePath)
		if err != nil {
			fmt.Printf("Could not read file: %v\n", err.Error())
			os.Exit(1)
		}
		defer file.Close()

		reader = file
	}

	if *bytesFlag {
		byteCount, err := countBytes(reader)
		if err != nil {
			fmt.Printf("Could not count bytes in file: %v\n", err.Error())
		}

		fmt.Printf("%v %v\n", byteCount, filePath)
	}

	if *newLineFlag {
		lineCount, err := count(reader, bufio.ScanLines)
		if err != nil {
			fmt.Printf("Could not count lines in file: %v\n", err.Error())
		}

		fmt.Printf("%v %v\n", lineCount, filePath)
	}

	if *wordFlag {
		wordCount, err := count(reader, bufio.ScanWords)
		if err != nil {
			fmt.Printf("Could not count words in file: %v\n", err.Error())
		}

		fmt.Printf("%v %v\n", wordCount, filePath)
	}

	if *characterFlag {
		runeCount, err := count(reader, bufio.ScanRunes)
		if err != nil {
			fmt.Printf("Could not count characters in file: %v\n", err.Error())
		}

		fmt.Printf("%v %v\n", runeCount, filePath)
	}

	if *bytesFlag == false && *newLineFlag == false && *wordFlag == false && *characterFlag == false {
		byteCount, err := countBytes(reader)
		if err != nil {
			fmt.Printf("Could not count bytes in file: %v\n", err.Error())
		}

		wordCount, err := count(reader, bufio.ScanWords)
		if err != nil {
			fmt.Printf("Could not count words in file: %v\n", err.Error())
		}

		lineCount, err := count(reader, bufio.ScanLines)
		if err != nil {
			fmt.Printf("Could not count lines in file: %v\n", err.Error())
		}

		fmt.Printf("%v %v %v %v\n", lineCount, wordCount, byteCount, filePath)
	}
}

func countBytes(reader io.ReadSeeker) (int, error) {
	byteCount := 0

	for {
		buffer := make([]byte, 1024)
		n, err := reader.Read(buffer)
		byteCount += n

		if err == io.EOF {
			break
		}
		if err != nil {
			return -1, err
		}
	}

	reader.Seek(0, 0)

	return byteCount, nil
}

func count(reader io.ReadSeeker, splitMode bufio.SplitFunc) (int, error) {
	lineCount := 0
	scanner := bufio.NewScanner(reader)
	scanner.Split(splitMode)

	for scanner.Scan() {
		lineCount++
	}

	if err := scanner.Err(); err != nil {
		return -1, err
	}

	reader.Seek(0, 0)

	return lineCount, nil
}
