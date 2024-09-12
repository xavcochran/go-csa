package main

import (
	"fmt"
	"time"
)

// slowSender sends a string every 2 seconds.
func slowSender(c chan<- string) {
	for {
		time.Sleep(2 * time.Second)
		c <- "I am the slowSender"
	}
}

// fastSender sends consecutive ints every 500 ms.
func fastSender(c chan<- int) {
	for i := 0; ; i++ {
		time.Sleep(500 * time.Millisecond)
		c <- i
	}
}

func fasterSender(c chan<- []int) {
	for i := 0; ; i++ {
		time.Sleep(200 * time.Millisecond)
		c <- []int{i, i + 1, i + 2}
	}
}

// main starts the two senders and then goes into an infinite loop of receiving their messages.
func main() {
	intSlice := make(chan []int, 10)
	go fasterSender(intSlice)
	ints := make(chan int, 10)
	go fastSender(ints)
	strings := make(chan string, 10)
	go slowSender(strings)

	for { // = while(true)
		select {
		case s := <-strings:
			fmt.Println("Received a string", s)
		case i := <-ints:
			fmt.Println("Received an int", i)
		case is := <-intSlice:
			fmt.Println("Received an int", is)
		default:
			fmt.Println("--- Nothing to receive, sleeping for 3s...")
			time.Sleep(3 * time.Second)
		}

	}
}
