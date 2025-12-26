use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use crate::app::{App, InputMode, TextInputType};

pub fn render_status_widget<'a>(app: &'a App) -> Paragraph<'a> {
    if let InputMode::TextInput(ref text_input) = app.input_mode {
        let prompt = match text_input.input_type {
            TextInputType::NPCName => "NPC Name: ",
            TextInputType::PCName => "PC Name: ",
        };
        let input_text = format!("{}{}_", prompt, text_input.buffer);
        Paragraph::new(input_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Input"))
    } else {
        // Show last 3 log messages in status bar
        let last_messages = app.last_log_messages(1);
        let log_text = if last_messages.is_empty() {
            "No messages".to_string()
        } else {
            last_messages.join(" | ")
        };
        Paragraph::new(log_text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Status (Press ` to view full log)"))
            .wrap(Wrap { trim: true })
    }
}

pub fn render_log_view<'a>(app: &'a App) -> Paragraph<'a> {
    let mut lines: Vec<Line> = vec![
        Line::from("Log Messages (Press ` to close)"),
        Line::from(""),
    ];
    
    if app.log.is_empty() {
        lines.push(Line::from("No log messages"));
    } else {
        for message in &app.log {
            lines.push(Line::from(message.as_str()));
        }
    }
    
    Paragraph::new(lines)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Log"))
        .wrap(Wrap { trim: true })
}