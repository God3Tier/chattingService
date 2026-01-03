use ratatui::{
    buffer::Buffer,
    layout::{Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Default)]
pub struct DisplayTextInput<'a> {
    title: &'a str
}

impl<'a> DisplayTextInput<'a> {
    pub fn new(title:&'a str) -> DisplayTextInput<'a> {
        DisplayTextInput {
            title
        }
    }
}

impl<'a> Widget for &DisplayTextInput<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(
            self.title
        )
        .bold();
        let main = Paragraph::new(title).block(Block::default().borders(Borders::ALL));
        main.render(area, buf);
    }
}
