#![feature(lang_items, llvm_asm)]
#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize};



use linux_kernel_module::{self, cstr, println};

pub mod msr;

//defining important constants
//static CLOS0_CPU_MASK: u32 = 0xfffeu32;
//static CLOS1_CPU_MASK: u32 = 0x1u32;
static CLOS0_BIT_MASK: u64 = 0b11111111110u64;
static CLOS1_BIT_MASK: u64 = 0b00000000001u64;
static DEFAULT_BIT_MASK: u64 = 0b11111111111u64;
static IA32_L3_CBM_BASE: u32 = 0xc90u32;
//static IA32_PQR_ASSOC_ID: u32 = 0xc8fu32;

struct WriteFile {
    written: AtomicUsize,
}

impl linux_kernel_module::file_operations::FileOperations for WriteFile {
    const VTABLE: linux_kernel_module::file_operations::FileOperationsVtable =
        linux_kernel_module::file_operations::FileOperationsVtable::builder::<Self>()
            .read()
            .write()
            .build();

    fn open() -> linux_kernel_module::KernelResult<Self> {
        return Ok(WriteFile {
            written: AtomicUsize::new(0),
        });
    }
}

impl linux_kernel_module::file_operations::Read for WriteFile {
    fn read(
        &self,
        buf: &mut linux_kernel_module::user_ptr::UserSlicePtrWriter,
        _offset: u64,
    ) -> linux_kernel_module::KernelResult<()> {
        let val = String::from("You read me!\n");
        buf.write(val.as_bytes())?;
        return Ok(());
    }
}

impl linux_kernel_module::file_operations::Write for WriteFile {
    fn write(
        &self,
        buf: &mut linux_kernel_module::user_ptr::UserSlicePtrReader,
        _offset: u64,
    ) -> linux_kernel_module::KernelResult<()> {
        let data = buf.read_all()?;
        let string = vec_to_string(data);
        let pid = match string.parse::<i32>(){
            Ok(num) => num,
            Err(_) => 
            {
                println!("Error: The value written to /dev/uncrashable is not a valid integer.");
                return Ok(());
            },
        };
        
        //At this point, we will assume that the pid we have read corresponds to a valid process
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe{
        //TODO: Use assembly instructions to write to RDT registers that control Intel CAT technologies to allocate an exclusive region of the cache for the process with the given PID
        //Defining the MSRs that we will use for the two CLOSs
            let IA32_L3_QOS_MASK_0 = msr::MSR::new(IA32_L3_CBM_BASE);
            let IA32_L3_QOS_MASK_1 = msr::MSR::new(IA32_L3_CBM_BASE + 1);

        //writing values to all cpu MSRs
            msr::write_all_cpus(IA32_L3_QOS_MASK_0, CLOS0_BIT_MASK);
            msr::write_all_cpus(IA32_L3_QOS_MASK_1, CLOS1_BIT_MASK);
            println!("Wrote to MSRs");

            //Assigning our task to the closid 1
            linux_kernel_module::bindings::assign_closid(pid as linux_kernel_module::c_types::c_int, 1 as u32);

        //Now, we will flush the entire cache using the wbinvd assembly instruction
        //WBINVD is described here: https://www.felixcloutier.com/x86/wbinvd
        
            llvm_asm!("WBINVD");
        }

        println!("Flushed cache!");


        //println!("Read this: {}", string);
        return Ok(());
    }
}

struct UncrashableModule {
    _chrdev_registration: linux_kernel_module::chrdev::Registration,
}

impl linux_kernel_module::KernelModule for UncrashableModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let chrdev_registration =
            linux_kernel_module::chrdev::builder(cstr!("uncrashablemodule"), 0..1)?
                .register_device::<WriteFile>()
                .build()?;

        //resetting CLOSids of CAT registers
        let IA32_L3_QOS_MASK_0 = msr::MSR::new(IA32_L3_CBM_BASE);
        let IA32_L3_QOS_MASK_1 = msr::MSR::new(IA32_L3_CBM_BASE + 1);
        unsafe{
            msr::write_all_cpus(IA32_L3_QOS_MASK_0, DEFAULT_BIT_MASK);
            msr::write_all_cpus(IA32_L3_QOS_MASK_1, DEFAULT_BIT_MASK);
        }

        println!("Successfuly initialized rust kernel module.");
        Ok(UncrashableModule {
            _chrdev_registration: chrdev_registration,
        })
    }
}

impl Drop for UncrashableModule{
    fn drop(&mut self){
        //resetting CLOSids of CAT registers
        let IA32_L3_QOS_MASK_0 = msr::MSR::new(IA32_L3_CBM_BASE);
        let IA32_L3_QOS_MASK_1 = msr::MSR::new(IA32_L3_CBM_BASE + 1);
        unsafe{
            msr::write_all_cpus(IA32_L3_QOS_MASK_0, DEFAULT_BIT_MASK);
            msr::write_all_cpus(IA32_L3_QOS_MASK_1, DEFAULT_BIT_MASK);
        }
        println!("Successfully unloaded rust kernel module.");
    }
}

//this function will take a vector of u8 integers and convert it into a string containing the
//corresponding ascii characters
fn vec_to_string (v: Vec<u8>) -> String {
    let mut s = alloc::string::String::new();
    for c in v{
        s.push(c as char);
    }
    s
}




linux_kernel_module::kernel_module!(
    UncrashableModule,
    author: "Jacob Earle",
    description: "A module for pinning an application to the L3 cache using Intel Cat",
    license: "GPL"
);

#[lang = "eh_personality"] extern fn eh_personality() {}
