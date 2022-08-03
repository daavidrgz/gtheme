use tui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};
use tui_logger::TuiLoggerWidget;

pub struct LoggerWidget<'a> {
    widget: TuiLoggerWidget<'a>,
}

impl<'a> LoggerWidget<'a> {
    pub fn new() -> Self {
        let tui_lg = TuiLoggerWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Gray))
            .style_info(Style::default().fg(Color::Blue))
            .output_separator('>')
            .output_timestamp(Some(String::from("%H:%M:%S")))
            .output_level(None)
            .output_target(false)
            .output_file(false)
            .output_line(false)
            .block(
                Block::default()
                    .title(" LOGS  ")
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Thick)
                    .borders(Borders::ALL),
            );

        LoggerWidget { widget: tui_lg }
    }

    pub fn get_widget(self) -> TuiLoggerWidget<'a> {
        self.widget
    }
}
