# Uncrashable Rust Kernel Module
This project aims to implement a kernel module for Linux in safe Rust that utilizes Intel CAT technology to isolate the program associated with a given PID in memory in the L3 cache of the host machine. In order to install the necessary dependencies for the project, as well as the appropriate nightly rust toolchain, simply run the setup script
```
chmod +x setup.sh
./setup.sh
```
in this directory on an installation of Ubuntu 20.04. After this, make sure that you have installed the linux headers corresponding to your kernel version.

## Requirements
1. An Intel Xeon processor with RDT (Resource Director Technology) + CAT (Cache Allocation Technology) enabled. To check whether your processor has the necessary features enabled, run
```
lscpu | grep Flags
``` 
and check that the output contains cat\_l3, cdp\_l3, occup\_llc, cqm\_mbm\_total, and cqm\_mbm\_local.

2. A linux kernel compiled with the flag **CONFIG\_INTEL\_RDT=y** (for 4.x kernels) or **CONFIG\_X86\_CPU\_RESCTRL** (for 5.x kernels). However, this project was built and tested on the kernel version 4.19.125 on Ubuntu 20.04, and this version is recommended, as the Rust kernel module framework can be somewhat buggy with 5.0+ kernels.

## Compiling and Running Project
After running setup.sh and ensuring that all the dependencies (especially the proper kernel headers) were properly installed, enter the project directory and compile the kernel module with the following commands
```
cd uncrashable_module
make clean
cargo clean
make
``` 
If this worked correctly, you should see that the kernel module compiled as uncrashablemodule.ko. Then, to quickly load the kernel module and mount its associated character device, run the provided script
```
chmod +x load_module.sh
./load_module.sh
```
To unload the module and remove the character device, run the provided script
```
chmod +x unload_module.sh
./unload_module.sh
```

## Additional Resources
To learn more about Intel cat: [https://software.intel.com/content/www/us/en/develop/articles/introduction-to-cache-allocation-technology.html](https://software.intel.com/content/www/us/en/develop/articles/introduction-to-cache-allocation-technology.html)

Here is a paper describing the framework used to build Linux kernel modules in Rust: [https://mssun.me/assets/ares19securing.pdf](https://mssun.me/assets/ares19securing.pdf)

Finally, the original code for the linux kernel module framework can be found in [this project](https://github.com/fishinabarrel/linux-kernel-module-rust) from fishinabarrel.

