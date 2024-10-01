package main

import (
	"container/list"
	"flag"
	"fmt"
	"math/rand"
	"sync/atomic"
	"time"

	"github.com/ChrisGora/semaphore"
)

var debug *bool
var activeTransactions int32
var maxActiveTransactions int32

type Txn struct {
	tx       transaction
	to, from *account
}

func active() {
	currentActive := atomic.AddInt32(&activeTransactions, 1)

	for {
		maxActive := atomic.LoadInt32(&maxActiveTransactions)
		if currentActive > maxActive {
			if atomic.CompareAndSwapInt32(&maxActiveTransactions, maxActive, currentActive) {
				break
			}
		} else {
			break
		}
	}
	fmt.Printf("Active Transactions: %d\n", currentActive)
}
func inactive() {
	atomic.AddInt32(&activeTransactions, -1)
}

// An executor is a type of a worker goroutine that handles the incoming transactions.
func executor(bank *bank, executorId int, transactionQueue chan Txn, done chan<- bool, sema semaphore.Semaphore) {
	for t := range transactionQueue {
		active()
		// Lock accounts

		from := bank.getAccountName(t.tx.from)
		to := bank.getAccountName(t.tx.to)

		fmt.Println("Executor\t", executorId, "processing transaction from", from, "to", to)

		e := bank.addInProgress(t.tx, executorId) // Removing this line will break visualisations.

		bank.execute(t.tx, executorId)

		if t.from.locked || t.to.locked {
			bank.unlockAccount(t.tx.from, "Manager")
			bank.unlockAccount(t.tx.to, "Manager")
		}
		sema.Post()

		bank.removeCompleted(e, executorId) // Removing this line will break visualisations.

		done <- true
		inactive()
	}
}

func manager(bank *bank, transactionQueue <-chan transaction, done chan bool, bankSize int) {
	transactions := make([]Txn, 0)
	accounts := make(map[int]*account)
	sema := semaphore.Init(1, 1)
	executorQueue := make(chan Txn, bankSize)

	for i, a := range bank.accounts {
		accounts[i] = a
	}
	for t := range transactionQueue {
		transactions = append(transactions, Txn{tx: t, from: accounts[t.from], to: accounts[t.to]})
	}

	for i := 0; i < bankSize; i++ {
		go executor(bank, i, executorQueue, done, sema)
	}



	for len(transactions) > 0 {
		sema.Wait()
		for i := len(transactions) - 1; i >= 0; i-- {
			t := transactions[i]

			if !t.from.locked && !t.to.locked {
				bank.lockAccount(t.tx.from, "Manager")
				bank.lockAccount(t.tx.to, "Manager")

				executorQueue <- t
				transactions = append(transactions[:i], transactions[i+1:]...) // gets start of list to i and then appends the rest of the list after i
			}
		}
	}

	
	close(executorQueue)

}

func toChar(i int) rune {
	return rune('A' + i)
}

// main creates a bank and executors that will be handling the incoming transactions.
func main() {
	rand.Seed(time.Now().UTC().UnixNano())
	debug = flag.Bool("debug", false, "generate DOT graphs of the state of the bank")
	flag.Parse()

	bankSize := 6 // Must be even for correct visualisation.
	transactions := 1000

	accounts := make([]*account, bankSize)
	for i := range accounts {
		accounts[i] = &account{name: string(toChar(i)), balance: 1000}
	}

	bank := bank{
		accounts:               accounts,
		transactionsInProgress: list.New(),
		gen:                    newGenerator(),
	}

	startSum := bank.sum()

	transactionQueue := make(chan transaction, transactions)
	expectedMoneyTransferred := 0
	for i := 0; i < transactions; i++ {
		t := bank.getTransaction()
		expectedMoneyTransferred += t.amount
		transactionQueue <- t
	}
	close(transactionQueue)

	done := make(chan bool)

	// for i := 0; i < bankSize; i++ {
	// 	go executor(&bank, i, transactionQueue, done)
	// }

	go manager(&bank, transactionQueue, done, bankSize)

	for total := 0; total < transactions; total++ {
		fmt.Println("Completed transactions\t", total)
		<-done
	}

	fmt.Println()
	fmt.Println("Expected transferred", expectedMoneyTransferred)
	fmt.Println("Actual transferred", bank.moneyTransferred)
	fmt.Println("Expected sum", startSum)
	fmt.Println("Actual sum", bank.sum())
	if bank.sum() != startSum {
		panic("sum of the account balances does not much the starting sum")
	} else if len(transactionQueue) > 0 {
		panic("not all transactions have been executed")
	} else if bank.moneyTransferred != expectedMoneyTransferred {
		panic("incorrect amount of money was transferred")
	} else {
		fmt.Println("The bank works!")
	}
}
