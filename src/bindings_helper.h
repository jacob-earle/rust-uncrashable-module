#include <linux/cdev.h>
#include <linux/fs.h>
#include <linux/module.h>
#include <linux/random.h>
#include <linux/slab.h>
#include <linux/uaccess.h>
#include <linux/version.h>
#include <linux/smp.h>
#include <asm/msr.h>



// Bindgen gets confused at certain things
//
const gfp_t BINDINGS_GFP_KERNEL = GFP_KERNEL;

//My addition to write to an MSR
//The wrapper around wrmsr_safe is necessary because Bindgen cannot create wrappings for inline functions
typedef struct c_msr {
    unsigned int msr;
    u32 low;
    u32 high;
} c_msr;

void my_wrmsr(void * arg){
    c_msr * args = (c_msr *) arg;
    wrmsr_safe(args->msr, args->low, args->high);
}