use std::collections::HashMap;


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

pub struct InterruptController {
    priority: Priority,
    pending: Vec<Exception>,
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
        }
    }

    pub fn throw(&mut self, exception: Exception) {
        self.pending.push(exception);
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

    pub fn handle(&mut self) {
        if let Some(exception) = self.poll() {
            match exception {
                Exception::Reset => todo!("reset"),
                _ => {
                },
            }
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


