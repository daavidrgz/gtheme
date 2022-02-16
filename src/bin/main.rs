use std::fs::{self,File};
use std::io::{self, BufRead};
use std::path::Path;
use std::{thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Paragraph, Wrap},
    layout::{Layout, Constraint, Direction, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn create_logo<'a>(logo_path:&'a str) -> Vec<Spans<'a>> {
    let logo_file = File::open(logo_path).expect(&format!("Error while opening logo file in {}", logo_path));
    let file_lines = io::BufReader::new(logo_file).lines();

    let colors = vec![Color::Red, Color::Green, Color::Yellow, Color::Blue, Color::Magenta, Color::Cyan];

    let mut spans: Vec<Spans> = vec![];

    for l in file_lines {
        let line = l.expect("Error while reading logo file");
        let words: Vec<&str> = line.split('$').collect();

        let mut line_spans: Vec<Span> = vec![];
        for (idx,word) in words.into_iter().enumerate() {
            line_spans.push(Span::styled(String::from(word), Style::default().fg(colors[idx])));
        }
        spans.push(Spans::from(line_spans));
    }
    spans
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(80)
                ].as_ref()
            )
            .split(f.size());

        let hBox = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(f.size());
        
        // f.render_widget(hBox, chunks[1]);

        let paragraph = Paragraph::new(create_logo("./assets/logo.txt"))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });
        f.render_widget(paragraph, chunks[0]);

        let block = Block::default()
                .title(" DESKTOPS ")
                .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })?;

    thread::sleep(Duration::from_millis(10000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
