
#include "channel.h"

#include <memory.h>
#include <stdio.h>
#include <stdlib.h>

struct chan_t {
    // TODO: Your channel struct here...
    pthread_mutex_t mutex;
    pthread_cond_t rcv_cond;
    pthread_cond_t snd_cond;
    int *buffer;
    bool recv_waiting;
    bool send_waiting;
};

typedef struct chan_t chan_t;

chan_t *chan_create() {
    chan_t *chan = (chan_t *)malloc(sizeof(chan_t));
    pthread_mutex_init(&chan->mutex, NULL);
    pthread_cond_init(&chan->rcv_cond, NULL);
    pthread_cond_init(&chan->snd_cond, NULL);
    
    return chan;
}

void chan_destroy(chan_t *chan) {
    free(chan);
}

void chan_send_int(chan_t *chan, int i) {
    // TODO: Sending logic
    pthread_mutex_lock(&chan->mutex);
    chan->buffer = (int *)malloc(sizeof(int));
    chan->buffer[0] = i;
    if (chan->recv_waiting)
    {
    pthread_cond_signal(&chan->rcv_cond);

    }
    while (chan->buffer == NULL)
    {
        pthread_cond_wait(&chan->rcv_cond, &chan->mutex);
    }
    pthread_mutex_unlock(&chan->mutex);
}

int chan_recv_int(chan_t *chan) {
    // TODO: Receiving logic
    pthread_mutex_lock(&chan->mutex);
    
    chan->recv_waiting = true;
    while (chan->buffer == NULL)
    {
        pthread_cond_wait(&chan->rcv_cond, &chan->mutex);
    }
    chan->recv_waiting = false;

    int i = chan->buffer[0];
    free(chan->buffer);
    chan->buffer = NULL;
    pthread_cond_signal(&chan->snd_cond);
    pthread_mutex_unlock(&chan->mutex);
    return i;
}