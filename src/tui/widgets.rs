use super::Processor;

use crate::processor::instruction::{Instruction, InstructionKind};
use crate::processor::registers::Registers;

use ratatui::style::palette::tailwind;
use ratatui::{prelude::*, widgets::*};
use ratatui::Frame;


const INFO: &str = "(Esc) quit | (Enter) Step | (Space) Start/Pause";


macro_rules! alternate {
    ($count:expr) => {
        match $count % 2 {
            0 => tailwind::SLATE.c950,
            _ => tailwind::SLATE.c900,
        }
    }
}

pub struct Widgets<'a, 'b> {
    frame: &'b mut Frame<'a>,
    registers: RegisterWidget,
    instruction: InstructionWidget,
}

impl<'a, 'b> Widgets<'a, 'b> {
    pub fn new(frame: &'b mut Frame<'a>, mut processor: Processor) -> Widgets<'a, 'b> {
        let registers = processor.registers.clone();

        let mut instructions = vec![Instruction { kind: InstructionKind::Undefined, addr: 0 }; 16];

        instructions.fill_with(|| processor.fetch());

        Widgets {
            frame,
            registers: RegisterWidget::new(registers.clone()),
            instruction: InstructionWidget::new(instructions),
        }
    }

    fn background(&mut self) {
        let block = Block::new()
            .bg(tailwind::SLATE.c950);

        self.frame.render_widget(block, self.frame.area());
    }

    fn border(&mut self, area: Rect) {
        let block = Block::bordered()
            .title("cortex-m4");

        self.frame.render_widget(block, area);
    }

    fn footer(&mut self, area: Rect) {
        let paragraph = Paragraph::new(Line::from(INFO))
            .fg(tailwind::SLATE.c200)
            .bg(tailwind::SLATE.c950)
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .fg(tailwind::BLUE.c400)
            );

        self.frame.render_widget(paragraph, area);
    }

    pub fn draw(&mut self) {
        self.background();

        let [_, main_layout, bottom_layout] = Layout::vertical([Constraint::Fill(1), Constraint::Length(21), Constraint::Fill(1)])
            .horizontal_margin(20)
            .areas(self.frame.area());

        let [footer_layout, _] = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
            .areas(bottom_layout);

        let [register_layout, instruction_layout] = Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(50)])
            .spacing(1)
            .areas(main_layout.inner(Margin::new(3, 2)));

        self.border(main_layout);

        self.frame.render_widget(self.registers.widget(), register_layout);

        self.frame.render_widget(self.instruction.widget(), instruction_layout);

        self.footer(footer_layout);
    }
}

pub struct RegisterWidget {
    registers: Registers,
}

impl RegisterWidget {
    pub fn new(registers: Registers) -> RegisterWidget {
        RegisterWidget {
            registers,
        }
    }

    pub fn widget(&self) -> impl Widget {
        let rows = self.registers.all()
            .iter()
            .enumerate()
            .map(|(register, value)| {
                let row = match register {
                    13 => Row::new([format!("r{}", register), format!("{}", value), String::from("Stack Pointer")]),
                    14 => Row::new([format!("r{}", register), format!("{}", value), String::from("Link Register")]),
                    15 => Row::new([format!("r{}", register), format!("{}", value), String::from("Program Counter")]),
                    _ => Row::new([format!("r{}", register), format!("{}", value), String::from("Generic")]),
                };

                row.fg(tailwind::SLATE.c200).bg(alternate!(register))
            })
            .collect::<Vec<Row>>();

        let widths = [
            Constraint::Length(20),
            Constraint::Length(15),
            Constraint::Length(20),
        ];

        Table::new(rows, widths)
            .header(
                Row::new(vec!["Register", "Value", "Type"])
                    .style(
                        Style::default()
                            .fg(tailwind::SLATE.c200)
                            .bg(tailwind::BLUE.c900)
                    )
                    .height(1)
            )
    }
}

pub struct InstructionWidget {
    instructions: Vec<Instruction>,
}

impl InstructionWidget {
    pub fn new(instructions: Vec<Instruction>) -> InstructionWidget {
        InstructionWidget {
            instructions,
        }
    }

    pub fn widget(&self) -> impl Widget {
        let rows = self.instructions.iter()
            .enumerate()
            .map(|(count, inst)| {
                Row::new([format!("{:#x?}", inst.addr), format!("{}", inst.kind)])
                    .fg(tailwind::SLATE.c200).bg(alternate!(count))
            })
            .collect::<Vec<Row>>();

        let widths = [
            Constraint::Length(20),
            Constraint::Length(20),
        ];

        Table::new(rows, widths)
            .header(
                Row::new(vec!["Address", "Instruction"])
                    .style(
                        Style::default()
                            .fg(tailwind::SLATE.c200)
                            .bg(tailwind::BLUE.c900)
                    )
                    .height(1)
            )
    }
}

pub fn draw<'a, 'b>(frame: &'b mut Frame<'a>, processor: Processor) {
    let mut widgets = Widgets::new(frame, processor);

    widgets.draw();
}


