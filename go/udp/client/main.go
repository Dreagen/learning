package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
	"os"
	"time"
)

const GREEN = "\x1b[32m";
const RESET = "\x1b[0m";

func main() {
	udpAddress, err := net.ResolveUDPAddr("udp", "localhost:8080")
	if err != nil {
		log.Fatal("Couldn't resolve address: ", err)
	}

	connection, err := net.DialUDP("udp", nil, udpAddress)
	if err != nil {
		log.Fatal("Couldn't connect to address: ", err)
	}

	defer connection.Close()

	fmt.Println("Starting talking to the UDP echo server")
	for {
		scanner := bufio.NewScanner(os.Stdin)
		scanner.Scan()
		message := scanner.Text()

		_, err = connection.Write([]byte(message))
		if err != nil {
			log.Printf("Failed to write: %s", err)
		}

		connection.SetReadDeadline(time.Now().Add(5 * time.Second))
		buffer := make([]byte, 1024)
		n, _, err := connection.ReadFromUDP(buffer)
		if err != nil {
			log.Printf("Read error: %s", err)
		}

		fmt.Printf("%s%s%s\n", GREEN, string(buffer[:n]), RESET)
	}

}
