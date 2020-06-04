pub struct MSR {
    id: u32,
}

impl MSR{
    pub fn new(i: u32) -> MSR{
        MSR{ id: i }
    }

    pub unsafe fn wrmsr(&mut self, low: u32, high: u32){
        llvm_asm!("wrmsr" :: "{ecx}" (self.id), "{eax}" (low), "{edx}" (high) : "memory" : "volatile");
    }
}