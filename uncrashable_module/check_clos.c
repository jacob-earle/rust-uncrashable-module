#include <stdio.h>
#include <sched.h>
#include <stdlib.h>
#include <pid.h>

//takes the task pid as an argument and checks the CLOSid associated with the task
int main(char * args[]){
  //parsing the first argument
  int pid_int = atoi(args[1]);
  if (pid_int == 0){
    printf("Could not parse process PID. Please enter a valid integer argument.\n");
    exit(1);
  }
  printf("Attempting to read CLOSid of task with pid %d.\n", pid_int);

  //finding the appropriate task structure
  pid_t pid = (pid_t) pid_int;
  struct task_struct * task = pid_task(find_vpid, PIDTYPE_PID);
  if(task == NULL){
    printf("Could not locate task. Exiting.\n");
  }

  //printing closid
  printf("The closid associated with this task is: %d.", task -> closid);
}
