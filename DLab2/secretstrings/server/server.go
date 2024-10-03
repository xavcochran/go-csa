package main

import (
	//	"errors"
	//	"flag"
	//	"fmt"
	//	"net"
	"flag"
	"fmt"
	"math/rand"
	"net"
	"net/rpc"
	"time"
	"uk.ac.bris.cs/distributed2/secretstrings/stubs"
	// "net/rpc"
)

/** Super-Secret `reversing a string' method we can't allow clients to see. **/
func ReverseString(s string, i int) string {
	time.Sleep(time.Duration(rand.Intn(i)) * time.Second)
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func PremiumReverseString(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

type SecretStringOperations struct{}

func (s *SecretStringOperations) Reverse(req stubs.Request, res *stubs.Response) (err error) {
	res.Message = ReverseString(req.Message, 10)
	return
}

func (s *SecretStringOperations) FastReverse(req stubs.Request, res *stubs.Response) (err error) {
	res.Message = PremiumReverseString(req.Message)
	return

}

func main() {
	portPtr := flag.String("port", ":8030", "port to listen on")
	flag.Parse()

    rpc.Register(&SecretStringOperations{})
	ln, err := net.Listen("tcp", *portPtr)
	if err != nil {
		fmt.Println("error creating listener on port", *portPtr)
	}
	defer ln.Close()

    rpc.Accept(ln)
}
