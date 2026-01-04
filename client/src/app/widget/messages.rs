use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::connected_room::InputMode;

pub struct Messages<'input_mode, 'messages, 'room_id, 'input> {
    input_mode: &'input_mode InputMode,
    messages: &'messages Vec<String>,
    room_id: &'room_id str,
    input: &'input str,
}

impl<'input_mode, 'messages, 'room_id, 'input> Messages<'input_mode, 'messages, 'room_id, 'input> {
    pub fn new(
        input_mode: &'input_mode InputMode,
        messages: &'messages Vec<String>,
        room_id: &'room_id str,
        input: &'input str,
    ) -> Messages<'input_mode, 'messages, 'room_id, 'input>
    where
        'messages: 'input_mode,
        'messages: 'room_id,
        'messages: 'input,
    {
        Messages {
            input_mode,
            messages,
            room_id,
            input,
        }
    }
}

impl<'input_mode, 'messages, 'room_id, 'input> Widget
    for &Messages<'input_mode, 'messages, 'room_id, 'input>
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical({
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
        });

        let [help_area, input_area, message_area] = vertical.areas(area);

        Paragraph::new(match self.input_mode {
            InputMode::Editing => "Press escape to return to normal mode",
            InputMode::Normal => "Press e to edit. Press q to join a different room",
        })
        .render(help_area, buf);

        let available_height = message_area.height.saturating_sub(2);

        let messages: Vec<String> = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| format!("{i}: {m}"))
            .collect();

        let visible_messages = if messages.len() > available_height as usize {
            messages[messages.len() - available_height as usize..].to_vec()
        } else {
            messages
        };

        Paragraph::new(visible_messages.join("\n"))
            .block(Block::default().borders(Borders::ALL).title(self.room_id))
            .wrap(ratatui::widgets::Wrap { trim: false })
            .render(message_area, buf);

        Paragraph::new(self.input)
            .block(Block::default().borders(Borders::ALL).title("Input"))
            .render(input_area, buf);
    }
}
