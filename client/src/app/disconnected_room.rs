use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::Line, widgets::{Block, Borders, Paragraph, StatefulWidget, Widget}};

#[derive(Debug)]
pub struct WaitingRoom {
    pub buttons: [Rect; 4]
}

impl WaitingRoom {
    pub fn new() -> WaitingRoom {
        WaitingRoom { buttons: () }
    }
}

impl StatefulWidget for &WaitingRoom {
    type State = WaitingRoom;
    
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut WaitingRoom) {
        let title = Line::from("Welcome to data leaker room messaging service").bold();
        let instructions = Line::from(
            vec![
                "Select a room to join".into(),
                "Room 1 : 1".black().bold(),
                "Room 2 : 2".blue().bold(),
                "Room 3 : 3".red().bold(),
                "Room 4 : 4".black().bold(),
                "Custom room : Coming soon".into()
            ]
        );
        
        let paragraph = Paragraph::new(title).block(
            Block::default().title(instructions)
                .borders(Borders::ALL)
        );
        
        
        paragraph.render(area, buf);
        
        
        let 

    }
} 