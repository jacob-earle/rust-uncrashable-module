#!/bin/bash
#This short script consolidates the commands needed to load the module and mount the character device associated with it
MODULE=uncrashablemodule
CHRDEV=/dev/uncrashable
sudo insmod $MODULE.ko
sudo mknod $CHRDEV c `grep $MODULE /proc/devices | grep -o [0-9]*` 2 
sudo chmod a+w $CHRDEV
