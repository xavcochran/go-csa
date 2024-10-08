#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

#include "channel.h"

// Receives numbers from the channel
void *receiver(void *args) {
    chan_t *c = (chan_t *)args;

    for (int i = 0; i < 50; i++) {
        int x = chan_recv_int(c);
        printf("RECEIVER \t Received %d (%d/50)\n", x, i + 1);
    }

    printf("RECEIVER \t Finished.\n");

    pthread_exit(NULL);
}

// Sends numbers down the channel
void *sender(void *args) {
    chan_t *c = (chan_t *)args;

    for (int i = 0; i < 50; i++) {
        printf("SENDER \t\t Sending %d\n", i);
        chan_send_int(c, i);
        printf("SENDER \t\t Sent %d successfully\n", i);

        struct timespec tim = {.tv_sec = 0, .tv_nsec = 500000000L};
        nanosleep(&tim, NULL);
    }

    printf("SENDER \t\t Finished\n");

    pthread_exit(NULL);
}

int main() {
    pthread_t send_thread1;
    pthread_t recv_thread1;

    // Initialise channel
    chan_t *chan = chan_create();

    // Start receiver thread
    if (pthread_create(&recv_thread1, NULL, receiver, chan)) {
        fprintf(stderr, "Error creating recv thread.\n");
        return 1;
    }

    // Start sender thread
    if (pthread_create(&send_thread1, NULL, sender, chan)) {
        fprintf(stderr, "Error creating send thread.\n");
        return 1;
    }

    // Join sender thread
    if (pthread_join(send_thread1, NULL)) {
        fprintf(stderr, "Error joining send thread.\n");
        return 1;
    }

    // Join receiver thread
    if (pthread_join(recv_thread1, NULL)) {
        fprintf(stderr, "Error joining recv thread.\n");
        return 1;
    }

    // Clean up channel
    chan_destroy(chan);
    return 0;
}