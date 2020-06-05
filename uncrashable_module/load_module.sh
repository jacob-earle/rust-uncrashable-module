#!/bin/bash
#This short script consolidates the commands needed to load the module and mount the character device associated with it
MODULE=uncrashablemodule
CHRDEV=/dev/uncrashable

#First we must check whether the load_wrmsr module is already inserted
if lsmod | grep "load_wrmsr" &> /dev/null ; then
	echo "The code of load_wrmsr is already loaded into the kernel. Loading the main module now."
else
	echo "The code of load_wrmsr is not yet loaded into the kernel. Loading that module first."
	sudo insmod ../src/load_wrmsr.ko
	echo "Code successfully inserted. Loading the main module now."
fi

sudo insmod $MODULE.ko
sudo mknod $CHRDEV c `grep $MODULE /proc/devices | grep -o [0-9]*` 0 
sudo chmod a+w $CHRDEV
