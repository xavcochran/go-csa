#ifndef CHANNEL_H
#define CHANNEL_H

#include <stdbool.h>
#include <pthread.h>

// Generic channel structure
typedef struct chan_t chan_t;

// Channel creation and destruction
chan_t *chan_create();
void chan_destroy(chan_t *chan);

// Channel integer messaging
void chan_send_int(chan_t *chan, int i);
int chan_recv_int(chan_t *chan);

#endif
