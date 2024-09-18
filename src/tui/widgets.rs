use super::{Command, Processor};

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

pub fn draw<'a>(frame: &mut Frame<'a>, command: Command, processor: Processor) {
    let [_, main_layout, bottom_layout] = Layout::vertical([Constraint::Fill(1), Constraint::Length(21), Constraint::Fill(1)])
        .horizontal_margin(20)
        .areas(frame.area());

    let [footer_layout, _] = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
        .areas(bottom_layout);

    let [register_layout, memory_layout] = Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(60)])
        .spacing(1)
        .areas(main_layout.inner(Margin::new(3, 2)));

    draw_background(frame);

    draw_border(frame, main_layout);

    draw_registers(frame, processor, register_layout);

    draw_footer(frame, footer_layout);
}

fn draw_footer<'a>(frame: &mut Frame<'a>, area: Rect) {
    let paragraph = Paragraph::new(Line::from(INFO))
        .fg(tailwind::SLATE.c200)
        .bg(tailwind::SLATE.c950)
        .centered()
        .block(
            Block::bordered()
                .border_type(BorderType::Double)
                .fg(tailwind::BLUE.c400)
        );

    frame.render_widget(paragraph, area);
}

fn draw_border<'a>(frame: &mut Frame<'a>, area: Rect) {
    let block = Block::bordered()
        .title("cortex-m4");

    frame.render_widget(block, area);
}

fn draw_background<'a>(frame: &mut Frame<'a>) {
    let block = Block::new()
        .bg(tailwind::SLATE.c950);

    frame.render_widget(block, frame.area());
}

fn draw_registers<'a>(frame: &mut Frame<'a>, processor: Processor, area: Rect) {
    let rows = processor.registers.all()
        .iter()
        .enumerate()
        .map(|(register, value)| {
            Row::new([format!("r{}", register), format!("{}", *value)])
                .fg(tailwind::SLATE.c200)
                .bg(alternate!(register))
        })
        .collect::<Vec<Row>>();

    let widths = [
        Constraint::Length(20),
        Constraint::Length(5),
    ];

    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["Register", "Value"])
                .style(
                    Style::default()
                        .fg(tailwind::SLATE.c200)
                        .bg(tailwind::BLUE.c900)
                )
                .height(1)
        );

    frame.render_widget(table, area);
}

fn draw_memory<'a>(frame: &mut Frame<'a>, processor: Processor, area: Rect) {
}


