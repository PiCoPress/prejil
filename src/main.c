#include <stdio.h>
#include <getopt.h>
#include "opt/subopt.h"

struct option long_opt[] =
{
    {"version", no_argument, 0, 'v'},
    {"help", no_argument, 0, 'h'},
    {0, 0, 0, 0},
};

int main(int argc, char* argv[])
{
    int ch, opt_index = 0;
    while((ch = getopt_long(argc, argv, "vh", long_opt, &opt_index)) != -1)
    {
        switch(ch)
        {
            case 'v':
                printf("prejil version: %s\n", version());
                break;
            case 'h':
                help();
                break;
            case '?':
                // dd
                break;
            default:
                break;
        }
    }
    return 0;
}
