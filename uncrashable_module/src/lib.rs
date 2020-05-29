#![feature(lang_items)]
#![no_std]

extern crate alloc;
use alloc::string::String;

use core::sync::atomic::{AtomicUsize};



use linux_kernel_module::{self, cstr, println};

struct CycleFile;

impl linux_kernel_module::file_operations::FileOperations for CycleFile {
    const VTABLE: linux_kernel_module::file_operations::FileOperationsVtable =
        linux_kernel_module::file_operations::FileOperationsVtable::builder::<Self>()
            .read()
            .build();

    fn open() -> linux_kernel_module::KernelResult<Self> {
        return Ok(CycleFile);
    }
}
impl linux_kernel_module::file_operations::Read for CycleFile {
    fn read(
        &self,
        buf: &mut linux_kernel_module::user_ptr::UserSlicePtrWriter,
        offset: u64,
    ) -> linux_kernel_module::KernelResult<()> {
        buf.write(b"You read me!")?;
        return Ok(());
    }
}

struct SeekFile;

impl linux_kernel_module::file_operations::FileOperations for SeekFile {
    const VTABLE: linux_kernel_module::file_operations::FileOperationsVtable =
        linux_kernel_module::file_operations::FileOperationsVtable::builder::<Self>()
            .seek()
            .build();

    fn open() -> linux_kernel_module::KernelResult<Self> {
        return Ok(SeekFile);
    }
}

impl linux_kernel_module::file_operations::Seek for SeekFile {
    fn seek(
        &self,
        _file: &linux_kernel_module::file_operations::File,
        _offset: linux_kernel_module::file_operations::SeekFrom,
    ) -> linux_kernel_module::KernelResult<u64> {
        return Ok(1234);
    }
}

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
        //let data = buf.read_all()?;
        //println!("Read this: {:?}", data);
        return Ok(());
    }
}

struct ChrdevTestModule {
    _chrdev_registration: linux_kernel_module::chrdev::Registration,
}

impl linux_kernel_module::KernelModule for ChrdevTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let chrdev_registration =
            linux_kernel_module::chrdev::builder(cstr!("uncrashablemodule"), 0..3)?
                .register_device::<CycleFile>()
                .register_device::<SeekFile>()
                .register_device::<WriteFile>()
                .build()?;
        Ok(ChrdevTestModule {
            _chrdev_registration: chrdev_registration,
        })
    }
}

linux_kernel_module::kernel_module!(
    ChrdevTestModule,
    author: "Fish in a Barrel Contributors",
    description: "A module for testing character devices",
    license: "GPL"
);

#[lang = "eh_personality"] extern fn eh_personality() {}
