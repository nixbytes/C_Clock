# C_Clock

Creat a simple clock in c using for loops. This C code represents a simple digital clock program that displays the current time in a 12-hour format. Here's a summary of its functionality:

    Header Files: The program includes standard library headers for input/output operations (stdio.h), general utilities (stdlib.h), and unistd.h for the usleep function.

    Main Function: The entry point of the program where execution begins.

    Variable Declaration: It declares integer variables hours, mins, and secs to store the current time and an integer delay set to 100 (milliseconds).

    User Input for Time: The program prompts the user to enter the current time in hours, minutes, and seconds (HH:MM:SS format).

    Input Validation: It checks if the entered time is valid. The hours should be between 1 and 12, and minutes and seconds should be within 0 to 59. If the input is invalid, it displays an error message and exits.

    Infinite Loop: The program enters an infinite loop (while (1)) to continuously update and display the time.

    Time Update Logic:
        Increments the secs variable each cycle.
        If secs exceeds 59, it resets to 0 and increments mins.
        If mins exceeds 59, it resets to 0 and increments hours.
        If hours exceeds 12, it resets to 1, following a 12-hour clock format.

    Display Time: It displays the updated time in the format HH:MM:SS.

    Delay: The program pauses for the specified delay using usleep, which is set to 100 milliseconds (multiplied by 1000 to convert to microseconds as required by usleep).

    Clear Screen: It uses the system("clear") command to clear the console screen, assuming a Unix-like system. This makes the clock appear to update in place.

    Return Statement: The program theoretically ends with return 0, but in practice, it will never reach this point due to the infinite loop, unless externally interrupted.

[Credit Source](https://www.youtube.com/watch?v=72fIizW3N-8)
