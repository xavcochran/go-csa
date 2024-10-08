#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

struct question
{
    char *question;
    char *answer;
};
typedef struct question question_t;

struct ask_args
{
    int score;
    // conditional thread to signal when questions are done
    pthread_cond_t *cond_thread;
    // mutex to protect score and finished flag
    pthread_mutex_t *mutex;
    // flag to signal when questions are done
    bool finished;
};
typedef struct ask_args ask_args_t;

struct timer_args
{   
    // conditional thread to signal when time is up
    pthread_cond_t *cond_thread;
    // mutex to protect finished flag
    pthread_mutex_t *mutex;
    // flag to signal when time is up
    bool finished;
};
typedef struct timer_args timer_args_t;

void *timer(void *args)
{
    timer_args_t *timer_args = args;
    sleep(5);

    // signal that time is up
    pthread_mutex_lock(timer_args->mutex);
    timer_args->finished = true;
    pthread_cond_signal(timer_args->cond_thread);
    pthread_mutex_unlock(timer_args->mutex);

    printf("\n5s have passed\n");
    pthread_exit(NULL);
}

void *ask(void *args)
{
    ask_args_t *ask_args = args;
    question_t questions[] = {
        {.question = "3*2", .answer = "6"},
        {.question = "50/10", .answer = "5"},
        {.question = "2+1+1+1", .answer = "5"},
        {.question = "3^3", .answer = "27"},
        {.question = "3+3", .answer = "6"},
        {.question = "4/2", .answer = "2"}};

    for (int i = 0; i < 6; i++)
    {
        char *question = questions[i].question;
        char *answer = questions[i].answer;
        printf("%s? ", question);

        char input[128];
        fgets(input, 128, stdin);

        // strip newline
        input[strcspn(input, "\n")] = '\0';

        if (!strcmp(input, answer))
        {
            // increment score
            pthread_mutex_lock(ask_args->mutex);
            ask_args->score++;
            pthread_mutex_unlock(ask_args->mutex);

            printf("Correct!\n");
        }
        else if (strcmp(input, answer))
        {
            printf("Incorrect :-(\n");
        }
    }
    // signal that questions are done
    pthread_cond_signal(ask_args->cond_thread);
    ask_args->finished = true;
}

int main(int argc, char const *argv[])
{
    pthread_cond_t cond_thread;
    pthread_mutex_t mutex;

    pthread_mutex_init(&mutex, NULL);
    pthread_cond_init(&cond_thread, NULL);

    pthread_t timer_thread;
    timer_args_t timer_args = {.cond_thread = &cond_thread, .mutex = &mutex};
    if (pthread_create(&timer_thread, NULL, timer, &timer_args))
    {
        printf("Error creating timer thread.\n");
    }

    int score = 0;

    ask_args_t ask_args = {.cond_thread = &cond_thread, .mutex = &mutex};

    pthread_t ask_thread;
    if (pthread_create(&ask_thread, NULL, ask, &ask_args))
    {
        printf("Error creating asker thread.\n");
    }

    // wait for questions or timer to finish
    while (!ask_args.finished && !timer_args.finished)
    {
        pthread_cond_wait(&cond_thread, &mutex);
    };

    if (timer_args.finished)
        printf("Time's up!\n");

    score = ask_args.score;
    printf("End of questions, final score %d\n", score);

    // cleanup
    pthread_cancel(timer_thread);
    pthread_join(timer_thread, NULL);
    pthread_mutex_destroy(&mutex);
    pthread_cond_destroy(&cond_thread);

    return 0;
}
