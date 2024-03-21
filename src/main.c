#include <stdio.h>
#include <unistd.h>
#include <getopt.h>

struct option long_opt[] =
{
    {"version", no_argument, NULL, 0},
};

int main(int argc, char* argv[])
{
    int optio;
    while((optio = getopt(argc, argv, "vh")) != -1)
    {
        switch(optio)
        {
            case 'v':
                printf("prejil version:", version());
                break;
            case 'h':
                help();
                break;
            case '?':
                printf("Unknown option: %c", optopt);
                break;
        }
    }
    return 0;
}
