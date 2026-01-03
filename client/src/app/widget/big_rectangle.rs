use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Default)]
pub struct RectangleInstructions;

impl Widget for &RectangleInstructions {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(
            "Welcome to data leaker room messaging service\nSelect a room to join,",
        )
        .bold();
        let main = Paragraph::new(title).block(Block::default().borders(Borders::ALL));
        main.render(area, buf);
    }
}
