#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

int main() {
    int hours, mins, secs;
    int delay = 100;

    // Prompt the user to set the time
    printf("Set time (HH:MM:SS): \n");
    scanf("%d:%d:%d", &hours, &mins, &secs);

    // Check for invalid input
    if (hours < 1 || hours > 12 || mins < 0 || mins > 59 || secs < 0 || secs > 59) {
        printf("ERROR! Invalid input.\n");
        exit(0);
    }

    while (1) {
        secs++;

        // Update minutes and reset seconds if necessary
        if (secs > 59) {
            mins++;
            secs = 0;
        }

        // Update hours and reset minutes if necessary
        if (mins > 59) {
            hours++;
            mins = 0;
        }

        // Reset hours if it exceeds 12 (12-hour clock format)
        if (hours > 12) {
            hours = 1;
        }

        // Display the clock
        printf("\n Clock :");
        printf("\n %02d:%02d:%02d", hours, mins, secs);

        // Sleep for the specified delay (in milliseconds)
        usleep(delay * 1000);

        // Clear the console screen (assumes a Unix-like system)
        system("clear");
    }

    return 0;
}

