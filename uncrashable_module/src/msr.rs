use linux_kernel_module::{c_types, bindings};

pub struct MSR {
    id: c_types::c_uint,
}

impl MSR{
    pub fn new(i: c_types::c_uint) -> MSR{
        MSR{ id: i }
    }
}

pub unsafe fn write_all_cpus(msr: MSR, low: u32, high: u32){
    let wait: c_types::c_int = 0;
    let mut info = linux_kernel_module::bindings::c_msr{
        msr: msr.id,
        low: low,
        high: high
    };
    //Relying on the C API to write to each register
    linux_kernel_module::bindings::on_each_cpu(Some(linux_kernel_module::bindings::my_wrmsr), &mut info as *mut _ as *mut c_types::c_void, wait);
}
