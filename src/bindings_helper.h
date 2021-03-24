#include <linux/cdev.h>
#include <linux/fs.h>
#include <linux/module.h>
#include <linux/random.h>
#include <linux/slab.h>
#include <linux/uaccess.h>
#include <linux/version.h>
#include <linux/smp.h>
#include <asm/msr.h>
#include <linux/sched.h>
#include <asm/intel_rdt_sched.h>



// Bindgen gets confused at certain things
//
const gfp_t BINDINGS_GFP_KERNEL = GFP_KERNEL;

//My addition to write to an MSR
//The wrapper around wrmsr_safe is necessary because Bindgen cannot create wrappings for inline functions
typedef struct c_msr {
    unsigned int msr;
    u64 val;
} c_msr;

//this is a wrapper around the wrmsr function defined in asm/msr.h so that we can call it on all cpus
void my_wrmsr(void * arg){
    c_msr * args = (c_msr *) arg;
    wrmsrl(args->msr, args->val);
    printk(KERN_INFO "Hello from wrmsr.\nWrote %llx to register %x.", args->val, args->msr);
}


//function that will assign the CLOSid specified in closid_new to the task corresponding to the pid pid_int
void assign_closid(int pid_int, u32 closid_new){
    pid_t pid = (pid_t) pid_int;
    struct task_struct * task = pid_task(find_vpid(pid), PIDTYPE_PID);
    if(task == NULL){
        //task could not be found, so ignore changing the closid
	printk("Could not locate task.");
        return;
    }
    printk("Successfully found task structure.");
    //write the new closid
    task -> closid = closid_new;
}

void check_closid(int pid_int){
  //finding the appropriate task structure
  pid_t pid = (pid_t) pid_int;
  struct task_struct * task = pid_task(find_vpid(pid), PIDTYPE_PID);
  if(task == NULL){
    printk(KERN_INFO "Could not locate task. Exiting.\n");
    return;
  }

  //returning closid
  printk(KERN_INFO "Task closid is now: %d", (task -> closid));
}
