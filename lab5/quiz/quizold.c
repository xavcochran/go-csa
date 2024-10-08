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
    question_t *question;
    pthread_cond_t *cond_thread;
    pthread_mutex_t *mutex;
    bool finished;
};
typedef struct ask_args ask_args_t;

struct timer_args
{
    pthread_cond_t *cond_thread;
    pthread_mutex_t *mutex;
    bool finished;
};
typedef struct timer_args timer_args_t;

void *timer(void *args)
{
    timer_args_t *timer_args = args;
    sleep(5);
    pthread_mutex_lock(timer_args->mutex);
    timer_args->finished = true;
    pthread_cond_signal(timer_args->cond_thread);
    pthread_mutex_unlock(timer_args->mutex);
    printf("5s have passed\n");
    pthread_exit(NULL);
}

void *ask(void *args)
{
    ask_args_t *ask_args = args;

    printf("%s? ", ask_args->question->question);

    char input[128];
    fgets(input, 128, stdin);

    // strip newline
    input[strcspn(input, "\n")] = '\0';

    if (!strcmp(input, ask_args->question->answer))
    {
        printf("Correct!\n");
        pthread_mutex_lock(ask_args->mutex);
        ask_args->score++;
        pthread_mutex_unlock(ask_args->mutex);
    }
    else
    {
        printf("Incorrect :-(\n");
    }
    pthread_mutex_lock(ask_args->mutex);
    ask_args->finished = true;
    pthread_cond_signal(ask_args->cond_thread);
    pthread_mutex_unlock(ask_args->mutex);
    pthread_exit(NULL);
}

int main(int argc, char const *argv[])
{
    pthread_cond_t cond_thread;
    pthread_mutex_t mutex;

    pthread_mutex_init(&mutex, NULL);
    pthread_cond_init(&cond_thread, NULL);

    question_t questions[] = {
        {.question = "3*2", .answer = "6"},
        {.question = "50/10", .answer = "5"},
        {.question = "2+1+1+1", .answer = "5"},
        {.question = "3^3", .answer = "27"},
        {.question = "3+3", .answer = "6"},
        {.question = "4/2", .answer = "2"}};

    pthread_t timer_thread;
    timer_args_t timer_args = {.cond_thread = &cond_thread, .mutex = &mutex};
    if (pthread_create(&timer_thread, NULL, timer, &timer_args))
    {
        printf("Error creating timer thread.\n");
    }

    int score = 0;
    for (int i = 0; i < 6; ++i)
    {

        ask_args_t ask_args = {.score = score, .question = &questions[i], .cond_thread = &cond_thread, .mutex = &mutex};

        pthread_t ask_thread;
        if (pthread_create(&ask_thread, NULL, ask, &ask_args))
        {
            printf("Error creating asker thread.\n");
        }

        if (pthread_join(ask_thread, NULL))
        {
            printf("Error joining asker thread.\n");
        }

        while (!ask_args.finished && !timer_args.finished)
        {
            pthread_cond_wait(&cond_thread, &mutex);
        };

        if (timer_args.finished)
        {
            printf("Time's up!\n");
            break;
        }
        score = ask_args.score;
    }

    printf("End of questions, final score %d\n", score);
    return 0;
}
