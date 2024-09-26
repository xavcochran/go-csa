package main

import (
	"bufio"
	"flag"
	"fmt"
	"net"
	"time"
)

type Message struct {
	sender  int
	message string
}

func handleError(err error) {
	// TODO: all
	// Deal with an error event.
}

func acceptConns(ln net.Listener, conns chan net.Conn) {
	conn, err := ln.Accept()
	if err != nil {
		fmt.Println("Error accepting connection")
		fmt.Println(err)
	}
	conns <- conn
}

func handleClient(clients map[int]net.Conn, clientid int, msgs chan Message) {
	// TODO: all
	// So long as this connection is alive:
	// Read in new messages as delimited by '\n's
	// Tidy up each message and add it to the messages channel,
	// recording which client it came from.
	client := clients[clientid]
	for {
		message, err := bufio.NewReader(client).ReadString('\n')
		if err != nil {
			if err.Error() == "EOF" {
				fmt.Println("Client disconnected")
				client.Close()
				delete(clients, clientid)
				fmt.Println("Number of clients remaining:", len(clients))
				break
			} else {
				fmt.Println("Error reading from client")
				fmt.Println(err)
				break
			}
		}
		if len(message) > 1 {
			fmt.Println("Message:", message)
			msgs <- Message{clientid, message}
		}
	}
}

func handleMessage(msg Message, clients map[int]net.Conn) {
	for id, conn := range clients {
		if id != msg.sender {
			fmt.Fprintln(conn, msg.message) // would actually use conn.Write here
		}
	}
}

func main() {
	// Read in the network port we should listen on, from the commandline argument.
	// Default to port 8030
	portPtr := flag.String("port", ":8030", "port to listen on")
	flag.Parse()

	//TODO Create a Listener for TCP connections on the port given above.
	ln, err := net.Listen("tcp", *portPtr)
	if err != nil {
		fmt.Println("error creating listener on port", *portPtr)
	}
	//Create a channel for connections
	conns := make(chan net.Conn)
	//Create a channel for messages
	msgs := make(chan Message)
	//Create a mapping of IDs to connections

	clients := make(map[int]net.Conn)

	//Start accepting connections
	go acceptConns(ln, conns)
	for {
		select {
		case conn := <-conns:
			//TODO Deal with a new connection
			// - assign a client ID
			// - add the client to the clients map
			// - start to asynchronously handle messages from this client
			client_id := int(time.Now().UnixMicro())
			clients[client_id] = conn
			fmt.Println("Number of clients connected:", len(clients), "Client ID:", client_id)
			go handleClient(clients, client_id, msgs)
			go acceptConns(ln, conns)
		case msg := <-msgs:
			//TODO Deal with a new message
			go handleMessage(msg, clients)
		}
	}
}
