#include<stdlib.h>
#include <stdio.h>
#include <unistd.h>


int main(){

    int hours,mins,secs;
    int delay = 1000;
    printf("Set time: \n");
    scanf("%d%d%d",&hours,&mins,&secs);

    if(hours>12 || mins > 60 || secs >60){
        printf("ERROR ! \n");
        exit(0);
    }

    while(1)
        secs++;
        if (secs>59){
            mins++; 
            secs = 0;
        
        }
        if (mins>59){
            hours++;
            mins = 0;
        
        }
        if(hours >12){
            hours = 1;
        }

        printf("\n Clock :");
        printf("\n %02d:%02d:%02d",hours,mins,secs);
        sleep(delay);
        system("cls");
}
