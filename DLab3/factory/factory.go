package main

import (
	"flag"
	"fmt"
	"net"
	"net/rpc"
	"sync"

	"uk.ac.bris.cs/distributed3/pairbroker/stubs"
)

type Factory struct{
	Pipe chan int
}

var Multiply = "Factory.Multiply"

// TODO: Define a Multiply function to be accessed via RPC.
// Check the previous weeks' examples to figure out how to do this.
func (f *Factory) Multiply(pair stubs.Pair, res *stubs.JobReport) (err error) {
	fmt.Println("result", pair.X*pair.Y)
	res.Result = pair.X * pair.Y
	f.Pipe<-res.Result
	return
}

func (f *Factory) Divide(pair stubs.Pair, res *stubs.JobReport) (err error) {
	fmt.Println("result", pair.X/pair.Y)
	res.Result = pair.X / pair.Y
	return
}

func pipeline(broker *rpc.Client, pipe <-chan int) {
	for {
		X := <-pipe
		Y := <-pipe
		pair := stubs.Pair{X: X, Y: Y}
		fmt.Println("calling divid for", X, "&", Y)
		broker.Call(stubs.Publish, "divide", pair)
	}
}

func main() {
	pAddr := flag.String("ip", "127.0.0.1:8080", "IP and port to listen on")
	brokerAddr := flag.String("broker", "127.0.0.1:8030", "Address of broker instance")
	flag.Parse()

	pipe := make(chan int)

	// Register the Factory type with the RPC server
	err := rpc.Register(&Factory{Pipe: pipe})
	if err != nil {
		fmt.Println("Error registering Factory:", err)
		return
	}

	// Dial the broker
	rpcB, err := rpc.Dial("tcp", *brokerAddr)
	if err != nil {
		fmt.Println("Error dialing broker:", err)
		return
	}
	defer rpcB.Close()

	// Create subscription request
	request := stubs.Subscription{
		Topic:          "multiply",
		FactoryAddress: *pAddr,
		Callback:       "Factory.Multiply",
	}

	requestD := stubs.Subscription{
		Topic:          "divide",
		FactoryAddress: *pAddr,
		Callback:       "Factory.Divide",
	}

	// Create response placeholder
	response := stubs.StatusReport{}
	wg := sync.WaitGroup{}
	wg.Add(1)
	go func() {
		defer wg.Done()
		ln, err := net.Listen("tcp", *pAddr)
		if err != nil {
			fmt.Println("Error creating listener on port", *pAddr, ":", err)
			return
		}
		defer ln.Close()

		fmt.Println("Factory server listening on", *pAddr)

		// Accept incoming connections
		rpc.Accept(ln)
	}()

	go pipeline(rpcB, pipe)

	// Call the broker's Subscribe method
	err = rpcB.Call("Broker.Subscribe", request, &response)
	if err != nil {
		fmt.Println("Error calling Broker.Subscribe:", err)
		return
	}
	err = rpcB.Call("Broker.Subscribe", requestD, &response)
	if err != nil {
		fmt.Println("Error calling Broker.Subscribe:", err)
		return
	}


	fmt.Println(pipe)
	wg.Wait()
	// Listen for incoming connections

}
