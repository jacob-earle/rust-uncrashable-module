#include <linux/module.h>	/* Needed by all modules */
#include <linux/kernel.h>	/* Needed for KERN_INFO */
#include "bindings_helper.h"

int init_module(void)
{
	printk(KERN_INFO "Loaded my_wrmsr wrapper function.\n");

	/* 
	 * A non 0 return means init_module failed; module can't be loaded. 
	 */
	return 0;
}

void cleanup_module(void)
{
	printk(KERN_INFO "Unloaded my_wrmsr wrapper function.\n");
}

EXPORT_SYMBOL(my_wrmsr);
EXPORT_SYMBOL(assign_closid);
EXPORT_SYMBOL(check_closid);
MODULE_LICENSE("GPL");
