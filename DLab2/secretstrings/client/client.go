package main

import (
	"bufio"
	"flag"
	"net/rpc"
	"os"

	//	"bufio"
	//	"os"
	"fmt"

	"uk.ac.bris.cs/distributed2/secretstrings/stubs"
)



func callReverse(rpc *rpc.Client) {
	reader := bufio.NewReader(os.Stdin)
	for {
		input, _ := reader.ReadString('\n')
		request := stubs.Request{Message: input}
		response := new(stubs.Response)
		err := rpc.Call(stubs.ReverseHandler, request, response)
		if err != nil {
			fmt.Println("Error calling reverse handler:", err)
			return
		}
		fmt.Println("Server responded with:", response.Message)
	}
}

func callPremiumReverse(rpc *rpc.Client) {
	reader := bufio.NewReader(os.Stdin)
	for {
		input, _ := reader.ReadString('\n')
		request := stubs.Request{Message: input}
		response := new(stubs.Response)
		err := rpc.Call(stubs.PremiumReverseHandler, request, response)
		if err != nil {
			fmt.Println("Error calling premium reverse handler:", err)
			return
		}
		fmt.Println("Server responded with:", response.Message)
	}
}

func main() {
	server := flag.String("server", "127.0.0.1:8030", "IP:port string to connect to as server")

	fmt.Println("Server: ", *server)
	//TODO: connect to the RPC server and send the request(s)
	callType := flag.String("type", "standard", "Type of call to make (standard/premium)")
	flag.Parse()

	rpc, err := rpc.Dial("tcp", *server)
	if err != nil {
		fmt.Println("Error dialing:", err)
		return
	}
	
	if *callType == "standard" {
		callReverse(rpc)
	} else if *callType == "premium" {
		callPremiumReverse(rpc)
	} else {
		fmt.Println("Invalid type input please use \"standard\" or \"premium\"")
		return
	}

}
