#!/bin/bash
#This short script condenses the commands needed to unload the module and remove its associated character device
MODULE=uncrashablemodule
CHRDEV=/dev/uncrashable
sudo rmmod $MODULE
sudo rm $CHRDEV
make clean
cargo clean
