use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use crate::app::{App, TextInputType};

pub fn render_status_widget<'a>(app: &'a App) -> Paragraph<'a> {
    if let Some(ref text_input) = app.text_input {
        let prompt = match text_input.input_type {
            TextInputType::NPCName => "NPC Name: ",
            TextInputType::PCName => "PC Name: ",
        };
        let input_text = format!("{}{}_", prompt, text_input.buffer);
        Paragraph::new(input_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Input"))
    } else {
        Paragraph::new(app.message.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Status"))
    }
}