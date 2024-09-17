use super::{Command, Processor};

use ratatui::{prelude::*, widgets::*};
use ratatui::Frame;


pub fn draw<'a>(frame: &mut Frame<'a>, command: Command, processor: Processor) {
    let rows = processor.registers.all()
        .iter()
        .enumerate()
        .map(|(register, value)| Row::new([format!("r{}", register), format!("{}", *value)]))
        .collect::<Vec<Row>>();

    let widths = [
        Constraint::Length(5),
        Constraint::Length(5),
    ];

    let table = Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::new().blue())
        .header(Row::new(vec!["register, value"]).bottom_margin(1));

    let [register_area] = Layout::horizontal([Constraint::Percentage(40)]).areas(frame.area());

    frame.render_widget(table, register_area);
}


