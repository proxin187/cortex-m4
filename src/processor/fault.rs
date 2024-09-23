use super::{Processor, Mode};
use super::decoder::BitVec;

use crate::bus::DataBus;

use std::collections::HashMap;


pub struct Frame {
    align: bool,
    ptr: u32,
}

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Exception {
    Reset,
    Nmi,
    HardFault,
    MemManage,
    BusFault,
    UsageFault,
    SVCall,
    DebugMonitor,
    PendSV,
    SysTick,
    Interrupt {
        offset: usize,
    },
}

impl Into<usize> for Exception {
    fn into(self) -> usize {
        match self {
            Exception::Reset => 1,
            Exception::Nmi => 2,
            Exception::HardFault => 3,
            Exception::MemManage => 4,
            Exception::BusFault => 5,
            Exception::UsageFault => 6,
            Exception::SVCall => 11,
            Exception::DebugMonitor => 12,
            Exception::PendSV => 14,
            Exception::SysTick => 15,
            Exception::Interrupt { offset } => offset,
        }
    }
}

#[derive(Clone)]
pub struct Priority {
    priorities: HashMap<Exception, i32>,
}

impl Priority {
    pub fn new(priorities: Vec<(Exception, i32)>) -> Priority {
        Priority {
            priorities: priorities.iter().map(|x| *x).collect::<HashMap<Exception, i32>>(),
        }
    }

    pub fn get(&self, exception: Exception) -> i32 {
        self.priorities.get(&exception).map(|x| *x).unwrap_or(0)
    }
}

#[derive(Clone)]
pub struct InterruptController {
    priority: Priority,
    pending: Vec<Exception>,
    group: i32,
}

impl InterruptController {
    pub fn new() -> InterruptController {
        let priorities = vec![
            (Exception::Reset, -3),
            (Exception::Nmi, -2),
            (Exception::HardFault, -1),
        ];

        InterruptController {
            priority: Priority::new(priorities),
            pending: Vec::new(),
            group: 0,
        }
    }

    pub fn throw(&mut self, exception: Exception) {
        if self.priority.get(exception) < self.group {
            self.pending.push(exception);
        }
    }

    pub fn poll(&mut self) -> Option<Exception> {
        self.pending.sort_by(|a, b| {
            if self.priority.get(*a) == self.priority.get(*b) {
                Into::<usize>::into(*b).cmp(&Into::<usize>::into(*a))
            } else {
                self.priority.get(*b).cmp(&self.priority.get(*a))
            }
        });

        self.pending.pop()
    }
}

impl Processor {
    pub fn frame(&mut self) -> Frame {
        let align = (self.registers.get(13, self.mode) & (1 << 2)) != 0;

        self.registers.set(13, |sp| (sp - 0x20) & ((1 << 2) ^ 0xFFFF_FFFF), self.mode);

        Frame {
            align,
            ptr: self.registers.get(13, self.mode),
        }
    }

    pub fn push_stack(&mut self) {
        let frame = self.frame();

        for (offset, register) in [0, 1, 2, 3, 12, 14, 15].iter().enumerate() {
            self.write::<u32>(frame.ptr as usize + (offset * 4), self.registers.get(*register, self.mode));
        }

        self.write::<u32>(frame.ptr as usize + 0x1c, (self.registers.psr.value & !(1 << 9)) | ((frame.align as u32) << 9));

        if self.mode == Mode::Handle {
            self.registers.set(14, |_| 0xfffffff1, self.mode);
        } else if !self.registers.control.stack {
            self.registers.set(14, |_| 0xfffffff9, self.mode);
        } else {
            self.registers.set(14, |_| 0xfffffffd, self.mode);
        }
    }

    pub fn pop_stack(&mut self, frame: Frame, exc_return: u32) {
        for (offset, register) in [0, 1, 2, 3, 12, 14, 15].iter().enumerate() {
            let value = self.read::<u32>(frame.ptr as usize + (offset * 4));

            self.registers.set(*register, |_| value, self.mode);
        }

        self.registers.psr.value = self.read::<u32>(frame.ptr as usize + 0x1c);

        match exc_return.get(0..4) {
            0b0001 | 0b1001 => {
                self.registers.sp.msp = (self.registers.sp.msp + 0x20) | (self.registers.psr.get(9) as u32) << 2;
            },
            0b1101 => {
                self.registers.sp.psp = (self.registers.sp.psp + 0x20) | (self.registers.psr.get(9) as u32) << 2;
            },
            _ => {},
        }
    }

    pub fn exception_entry(&mut self, exception: Exception) {
        self.mode = Mode::Handle;

        self.registers.control.stack = false;

        self.registers.psr.value |= Into::<usize>::into(exception) as u32 & 0xff;

        let handler = self.read::<u32>(self.registers.vtor.addr() as usize + Into::<usize>::into(exception) * 4);

        self.registers.set(15, |_| handler, self.mode);

        if (handler & 1) != 0 {
            self.registers.psr.set(24);
        }
    }

    pub fn exception_return(&mut self, exc_return: u32) {
        match exc_return.get(0..4) {
            0b0001 | 0b1001 => {
                self.mode = exc_return.get(0..4)
                    .eq(&0b0001)
                    .then(|| Mode::Handle)
                    .unwrap_or(Mode::Thread);

                self.registers.control.stack = false;

                self.pop_stack(Frame {
                    align: false,
                    ptr: self.registers.sp.msp,
                }, exc_return);
            },
            0b1101 => {
            },
            _ => {
                self.nvic.throw(Exception::UsageFault);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interrupt_controller() {
        let mut nvic = InterruptController::new();

        nvic.throw(Exception::Nmi);
        nvic.throw(Exception::HardFault);
        nvic.throw(Exception::Reset);
        nvic.throw(Exception::BusFault);
        nvic.throw(Exception::SysTick);
        nvic.throw(Exception::MemManage);

        assert_eq!(Some(Exception::Reset), nvic.poll());
        assert_eq!(Some(Exception::Nmi), nvic.poll());
        assert_eq!(Some(Exception::HardFault), nvic.poll());
        assert_eq!(Some(Exception::MemManage), nvic.poll());
        assert_eq!(Some(Exception::BusFault), nvic.poll());
        assert_eq!(Some(Exception::SysTick), nvic.poll());
    }
}


