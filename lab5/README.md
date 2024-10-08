# Concurrency Lab 3

> If you're stuck look at docs on [man7.org](http://man7.org/linux/man-pages/man7/pthreads.7.html)

## Using the lab sheet

There are two ways to use the lab sheet, you can either:

- [create a new repo from this template](https://github.com/UoB-CSA/concurrency-lab-3/generate) - **this is the recommended way**
- download a [zip file](https://github.com/UoB-CSA/concurrency-lab-3/archive/master.zip)

Each question is rated to help you balance your work:

- :red_circle::white_circle::white_circle::white_circle::white_circle: - Easy, strictly necessary.
- :red_circle::red_circle::white_circle::white_circle::white_circle: - Medium, still necessary.
- :red_circle::red_circle::red_circle::white_circle::white_circle: - Hard, necessary if you're aiming for higher marks.
- :red_circle::red_circle::red_circle::red_circle::white_circle: - Hard, useful for coursework extensions.
- :red_circle::red_circle::red_circle::red_circle::red_circle: - Hard, beyond what you need for any part of the coursework.

## Question 1 - Hello World from pthreads :red_circle::red_circle::red_circle::red_circle::red_circle:

Open `hello.c`. It's a sample Hello World program that prints `"Hello World"` using a pthread.

### Create a new thread

The `pthread_create()` function starts a new thread in the calling process.

```c
pthread_create(&thread, NULL, hello_world, &n)
```

1) `pthread_t *thread` : The pointer to the pthread struct for the thread we are creating.
2) `const pthread_attr_t *attr` : The pointer to the pthread creation attributes. We pass `NULL` here to use the defaults.
3) `void *(* start_routine)(void *)` : The pointer to the start routine for the thread. This routine must take one `void *` argument and return a `void *`.
4) `void *arg` : The pointer to the argument to pass to this thread. Multiple arguments can passed with a struct.

### Wait for a thread to finish execution

The pthread_join() function waits for the thread specified to terminate. If that thread has already terminated, then pthread_join() returns immediately. The thread specified must be joinable.

```c
pthread_join(thread, NULL)
```

1) `pthread_t thread` : The thread to join.
2) `void **retval` : The pointer to a place to store the exit value of the thread.

### Task

Run `hello.c` and verify that a message `"Hello from thread 1"` is printed. Modify the provided code so that 5 pthreads are started and they all print a hello message. Example output:

```bash
Hello from thread 2
Hello from thread 3
Hello from thread 1
Hello from thread 4
Hello from thread 5
```

<details>
    <summary>Hint 1</summary>

You should only need to modify the `main()` function.

</details>

<details>
    <summary>Hint 2</summary>

Be careful with your memory management. Make sure that the value the argument pointer points to is still valid when the thread tries to access it.

</details>

## Question 2 - C Quiz :red_circle::red_circle::red_circle::red_circle::red_circle:

Open `quiz.c`. It's the quiz program from Lab 3. However, it is not complete. The skeleton you are given asks all the questions using pthreads and also starts the timer thread, but when 5s have passed the program does not terminate like it should.

Use mutexes and condition variables (*not* busy waiting) to fix the quiz program.

Here are some functions that you may find useful:

- `pthread_mutex_init`
- `pthread_mutex_lock`
- `pthread_mutex_unlock`
- `pthread_cond_init`
- `pthread_cond_wait`
- `pthread_cond_signal`

<details>
    <summary>Hint 1</summary>

You should no longer be using `pthread_join`.

</details>

<details>
    <summary>Hint 2</summary>

Our solution uses 1 mutex and 1 condition variable. You can use one condition variable to wait on both of your conditions (timer and quiz answer).

</details>

<details>
    <summary>Hint 3</summary>

Your timer should notify your condition variable when the 5s have elapsed.

</details>

## Question 3 - Synchronous channels :red_circle::red_circle::red_circle::red_circle::red_circle:

### Question 3a

Implement a simple channel using mutexes and condition variables.

- The channel should be for integers only.
- The channels should be synchronous. The sender must block and wait for a corresponding receive. The receiver must block and wait for a corresponding send.
- You can assume only one sender thread and one receiver thread.

You've been provided with `example_3a.c` which uses your channel implementation to send 50 integers from a sender thread to a receiver thread. You are also given a header file `channel.h` and a basic skeleton `channel.c`.

<details>
    <summary>Hint 1</summary>

This is easily achievable with 1 mutex and 2 condition variables.

</details>

<details>
    <summary>Hint 2</summary>

Use one condition variable for when the sender is waiting for the receiver. Another for when a receiver is waiting for a sender.

Use a mutex to protect your struct from race conditions.

</details>

### OPTIONAL Question 3b

Expand your simple channel to allow for multiple senders and receivers. You can also try to make your channel work with any data type.

Consider what should happen when two threads try to send at the same time.
Consider what should happen when two threads try to receive at the same time.

To verify your solution run `example_3b.c`. This program uses 3 senders and 3 receivers to exchange a total of 150 integers.

<details>
    <summary>Hint 1</summary>

You should keep track of how many threads are trying to send to the channel and how many threads are trying to receive from the channel.

</details>

<details>
    <summary>Hint 2</summary>

To support any data type allow copying of any `void *` to your channel bufffer. Keep track of the length of the contents in the buffer. Your receive function needs to say how many bytes have been received.

</details>
