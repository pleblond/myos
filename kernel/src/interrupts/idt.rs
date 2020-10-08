use core::marker::PhantomData;
use crate::paging::{PhysicalAddress, VirtualAddress};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct EntryOptions(u16);

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Entry<F> {
    pointer_low:    u16,
    gdt_selector:   u16,
    options:        EntryOptions,
    pointer_middle: u16,
    pointer_high:   u32,
    reserved:       u32,
    phantom:        PhantomData<F>,
}

#[repr(C, align(16))]
pub struct InterruptDescriptorTable {
    pub divide_error:                   Entry<HandlerFunc>,                     // 0x00
    pub debug:                          Entry<HandlerFunc>,                     // 0x01
    pub non_maskable_interrupt:         Entry<HandlerFunc>,                     // 0x02
    pub breakpoint:                     Entry<HandlerFunc>,                     // 0x03
    pub overflow:                       Entry<HandlerFunc>,                     // 0x04
    pub bound_range_exceeded:           Entry<HandlerFunc>,                     // 0x05
    pub invalid_opcode:                 Entry<HandlerFunc>,                     // 0x06
    pub device_not_available:           Entry<HandlerFunc>,                     // 0x07
    pub double_fault:                   Entry<DivergingHandlerFuncWithErrCode>, // 0x08
    pub coprocessor_segment_overrun:    Entry<HandlerFunc>,                     // 0x09
    pub invalid_tss:                    Entry<HandlerFuncWithErrCode>,          // 0x0A
    pub segment_not_present:            Entry<HandlerFuncWithErrCode>,          // 0x0B
    pub stack_segment_fault:            Entry<HandlerFuncWithErrCode>,          // 0x0C
    pub general_protection_fault:       Entry<HandlerFuncWithErrCode>,          // 0x0D
    pub page_fault:                     Entry<PageFaultHandlerFunc>,            // 0x0E
        reserved_0x0f:                  Entry<HandlerFunc>,                     // 0x0F
    pub x87_floating_point:             Entry<HandlerFunc>,                     // 0x10
    pub alignment_check:                Entry<HandlerFuncWithErrCode>,          // 0x11
    pub machine_check:                  Entry<DivergingHandlerFunc>,            // 0x12
    pub simd_floating_point:            Entry<HandlerFunc>,                     // 0x13
    pub virtualization:                 Entry<HandlerFunc>,                     // 0x14
        reserved_0x15:                  Entry<HandlerFunc>,                     // 0x15
        reserved_0x16:                  Entry<HandlerFunc>,                     // 0x16
        reserved_0x17:                  Entry<HandlerFunc>,                     // 0x17
        reserved_0x18:                  Entry<HandlerFunc>,                     // 0x18
        reserved_0x19:                  Entry<HandlerFunc>,                     // 0x19
        reserved_0x1a:                  Entry<HandlerFunc>,                     // 0x1A
        reserved_0x1b:                  Entry<HandlerFunc>,                     // 0x1B
        reserved_0x1c:                  Entry<HandlerFunc>,                     // 0x1C
        reserved_0x1d:                  Entry<HandlerFunc>,                     // 0x1D
    pub security_exception:             Entry<HandlerFuncWithErrCode>,          // 0x1E
        reserved_0x1f:                  Entry<HandlerFunc>,                     // 0x1F
    pub interrupts:                     [Entry<HandlerFunc>; 256 - 32],         // 0x20-0xFF
}

#[derive(Clone)]
#[repr(C)]
pub struct InterruptStackFrameValue {
    pub instruction_pointer: VirtualAddress,
    pub code_segment:        u64,
    pub cpu_flags:           u64,
    pub stack_pointer:       VirtualAddress,
    pub stack_segment:       u64,
}

#[repr(C)]
pub struct InterruptStackFrame {
    value: InterruptStackFrameValue,
}

pub type HandlerFunc =
    extern "x86-interrupt" fn(&mut InterruptStackFrame);
pub type HandlerFuncWithErrCode =
    extern "x86-interrupt" fn(&mut InterruptStackFrame, error_code: u64);
pub type PageFaultHandlerFunc =
    extern "x86-interrupt" fn(&mut InterruptStackFrame, error_code: u64);
pub type DivergingHandlerFunc =
    extern "x86-interrupt" fn(&mut InterruptStackFrame) -> !;
pub type DivergingHandlerFuncWithErrCode =
    extern "x86-interrupt" fn(&mut InterruptStackFrame, error_code: u64) -> !;
