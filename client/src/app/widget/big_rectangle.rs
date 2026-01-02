use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::Stylize, text::{Line, Text}, widgets::{Block, Borders, Paragraph, Widget}};

#[derive(Default)]
pub struct RectangleInstructions;

impl Widget for &RectangleInstructions {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Welcome to data leaker room messaging service").bold();
        let instructions = Line::from(
            vec![
                "Select a room to join".into(),
                "Room 1 : 1".black().bold(),
                "Room 2 : 2".blue().bold(),
                "Room 3 : 3".red().bold(),
                "Room 4 : 4".black().bold(),
                "Select r to sign in".bold(),
                "Custom room : Coming soon".into()
            ]
        );

        let paragraph = Paragraph::new(title).block(
            Block::default().title(instructions)
                .borders(Borders::ALL)
        );

        paragraph.render(area, buf);
    }
}