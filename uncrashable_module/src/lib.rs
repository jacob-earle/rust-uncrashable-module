#![feature(lang_items)]
#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize};



use linux_kernel_module::{self, cstr, println};

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
                -1
            },
        };
        if let Ok(val) = check_process_status(pid) {
            if val {
                println!("{} is an active process in the default resource group.", pid);
            }
            else {
                println!("{} is not an active process in the default resource group.", pid);
            }
        }
        else {
            println!("ERROR: Could not open /sys/fs/resctrl/tasks.");
        }
        //println!("Read this: {}", string);
        return Ok(());
    }
}

struct ChrdevTestModule {
    _chrdev_registration: linux_kernel_module::chrdev::Registration,
}

impl linux_kernel_module::KernelModule for ChrdevTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let chrdev_registration =
            linux_kernel_module::chrdev::builder(cstr!("uncrashablemodule"), 0..1)?
                .register_device::<WriteFile>()
                .build()?;
        Ok(ChrdevTestModule {
            _chrdev_registration: chrdev_registration,
        })
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

//This function will open the file /sys/fs/resctrl/tasks in order to check whether the process corresponding to pid is currently running and assigned to the default resctrl resource group
fn check_process_status(pid: i32) -> linux_kernel_module::KernelResult<bool> {
    //attempting to read and open /sys/fs/resctrl
    //let contents = 

    Ok(false)
}



linux_kernel_module::kernel_module!(
    ChrdevTestModule,
    author: "Fish in a Barrel Contributors",
    description: "A module for testing character devices",
    license: "GPL"
);

#[lang = "eh_personality"] extern fn eh_personality() {}
