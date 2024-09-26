package main

import (
	"bufio"
	"flag"
	"fmt"
	"net"
	"os"
)

func read(conn net.Conn, shouldExit *bool) {
	//TODO In a continuous loop, read a message from the server and display it.
	reader := bufio.NewReader(conn)
	for {

		message, err := reader.ReadString('\n')
		if err != nil {
			fmt.Println("Error reading from server")
			if err.Error() == "EOF" {
				fmt.Println("Server disconnected")
			} else {
				fmt.Println(err)
			}
			*shouldExit = true
			break
		}
		if message == "exit\n" {
			*shouldExit = true
			break
		}
		if len(message) > 1 {
			fmt.Println(message)
		}
	}
}

func write(conn net.Conn) {
	//TODO Continually get input from the user and send messages to the server.
	reader := bufio.NewReader(os.Stdin)
	for {

		text, _ := reader.ReadString('\n')
		fmt.Fprintf(conn, text)
	}
}

func main() {
	// Get the server address and port from the commandline arguments.
	addrPtr := flag.String("ip", "127.0.0.1:8030", "IP:port string to connect to")
	flag.Parse()
	//TODO Try to connect to the server
	//TODO Start asynchronously reading and displaying messages
	//TODO Start getting and sending user messages.
	conn, err := net.Dial("tcp", *addrPtr)
	if err != nil {
		fmt.Println("Error connecting to server")
		fmt.Println(err)
		return
	}
	fmt.Println(conn.RemoteAddr())
	shouldExit := false
	go write(conn)
	for {
		if shouldExit {
			break
		}
		read(conn, &shouldExit)
	}

}
