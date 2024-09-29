package main

import (
	"fmt"
	"sync"
)

func main() {
	sum := 0
	mu := sync.Mutex{}
	var wg sync.WaitGroup
	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			mu.Lock()
			sum = sum + 1
			mu.Unlock()
			wg.Done()
		}()
	}

	wg.Wait()
	fmt.Println(sum)
}
