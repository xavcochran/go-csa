#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

#include "channel.h"

struct sender_args_t {
    chan_t *c;
    int start;
    int delta;
    int sleep_duration;
};

typedef struct sender_args_t sender_args_t;

// Receives numbers from the channel
void *receiver(void *args) {
    chan_t *c = (chan_t *)args;

    for (int i = 0; i < 50; ++i) {
        int x = chan_recv_int(c);
        printf("RECEIVER \t Received %d (%d/50)\n", x, i + 1);
    }

    printf("RECEIVER \t Finished.\n");

    pthread_exit(NULL);
}

// Sends numbers down the channel
void *sender(void *args) {
    sender_args_t *sender_args = (sender_args_t *)args;
    chan_t *c = sender_args->c;
    int next_message = sender_args->start;
    int delta = sender_args->delta;

    for (int i = 0; i < 50; i++) {
        printf("SENDER %d \t Sending %d\n", delta, next_message);
        chan_send_int(c, next_message);
        printf("SENDER %d \t Sent %d successfully\n", delta, next_message);
        next_message += delta;

        struct timespec tim = {.tv_sec = 0, .tv_nsec = 10000000L * sender_args->sleep_duration};
        nanosleep(&tim, NULL);
    }

    printf("SENDER %d \t Finished\n", delta);

    pthread_exit(NULL);
}

int main() {
    pthread_t send_thread1;
    pthread_t send_thread2;
    pthread_t send_thread3;
    pthread_t recv_thread1;
    pthread_t recv_thread2;
    pthread_t recv_thread3;

    // Initialise channel
    chan_t *chan = chan_create();

    // Start receiver thread
    if (pthread_create(&recv_thread1, NULL, receiver, chan)) {
        fprintf(stderr, "Error creating recv thread.\n");
        return 1;
    }

    // Start receiver thread
    if (pthread_create(&recv_thread2, NULL, receiver, chan)) {
        fprintf(stderr, "Error creating recv thread.\n");
        return 1;
    }

    // Start receiver thread
    if (pthread_create(&recv_thread3, NULL, receiver, chan)) {
        fprintf(stderr, "Error creating recv thread.\n");
        return 1;
    }

    sender_args_t send_thread1_args = {
        .c = chan,
        .start = 0,
        .delta = 1,
        .sleep_duration = 3};

    // Start sender thread
    if (pthread_create(&send_thread1, NULL, sender, &send_thread1_args)) {
        fprintf(stderr, "Error creating send thread.\n");
        return 1;
    }

    sender_args_t send_thread2_args = {
        .c = chan,
        .start = 100,
        .delta = -1,
        .sleep_duration = 1};

    // Start sender thread
    if (pthread_create(&send_thread2, NULL, sender, &send_thread2_args)) {
        fprintf(stderr, "Error creating send thread.\n");
        return 1;
    }

    sender_args_t send_thread3_args = {
        .c = chan,
        .start = 1000,
        .delta = -2,
        .sleep_duration = 2};

    // Start sender thread
    if (pthread_create(&send_thread3, NULL, sender, &send_thread3_args)) {
        fprintf(stderr, "Error creating send thread.\n");
        return 1;
    }

    // Join receiver thread
    if (pthread_join(recv_thread1, NULL)) {
        fprintf(stderr, "Error joining recv thread.\n");
        return 1;
    }

    // Join receiver thread
    if (pthread_join(recv_thread2, NULL)) {
        fprintf(stderr, "Error joining recv thread.\n");
        return 1;
    }

    // Join receiver thread
    if (pthread_join(recv_thread3, NULL)) {
        fprintf(stderr, "Error joining recv thread.\n");
        return 1;
    }

    // Clean up channel
    chan_destroy(chan);
    return 0;
}