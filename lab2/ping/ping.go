package main

import (
	"fmt"
	"log"
	"os"
	"runtime/trace"
	"time"
)

func foo(channel chan string) {
	// TODO: Write an infinite loop of sending "pings" and receiving "pongs"
	for {
		channel <- "ping"
		fmt.Println("foo is sending: ping")
		<-channel
		fmt.Println("foo is receiving: pong")
	}
}

func bar(channel chan string) {
	// TODO: Write an infinite loop of receiving "pings" and sending "pongs"
	for {
		<-channel
		fmt.Println("bar is receiving: ping")
		channel <- "pong"
		fmt.Println("bar is sending: pong")
		fmt.Println()
	}
}

func pingPong() {
	pingPongChannel := make(chan string)
	go foo(pingPongChannel) // Nil is similar to null. Sending or receiving from a nil chan blocks forever.
	go bar(pingPongChannel)
	time.Sleep(500 * time.Millisecond)
}

func main() {
	f, err := os.Create("trace.out")
	if err != nil {
		log.Fatalf("failed to create trace output file: %v", err)
	}
	defer func() {
		if err := f.Close(); err != nil {
			log.Fatalf("failed to close trace file: %v", err)
		}
	}()

	if err := trace.Start(f); err != nil {
		log.Fatalf("failed to start trace: %v", err)
	}
	defer trace.Stop()

	pingPong()
}
