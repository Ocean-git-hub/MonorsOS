#[inline]
pub fn read_cr0() -> u64 {
    let cr0;
    unsafe {
        asm!("mov {}, cr0", out(reg) cr0);
    }
    cr0
}

#[inline]
pub unsafe fn write_cr0(cr0: u64) {
    asm!("mov cr0, {}", in(reg) cr0);
}

#[inline]
pub fn read_cr3() -> u64 {
    let cr3;
    unsafe {
        asm!("mov {}, cr3", out(reg) cr3);
    }
    cr3
}

#[inline]
pub unsafe fn write_cr3(cr3: u64) {
    asm!("mov cr3, {}", in(reg) cr3);
}

#[inline]
pub fn halt() {
    unsafe {
        asm!("hlt", options(nomem, nostack));
    }
}

#[inline]
pub fn nop() {
    unsafe {
        asm!("nop", options(nomem, nostack));
    }
}
