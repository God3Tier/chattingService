
use ratatui::{layout::{Constraint, Layout}, widgets::{Block, Borders, Paragraph, Widget}};

pub struct FillingButtons {
    instructions: Vec<String>, 
    num_rows: usize,
    num_cols: usize
}

impl FillingButtons {
    pub fn new(instructions: Vec<String>, num_rows: usize, num_cols: usize) -> FillingButtons {
        FillingButtons {
            instructions,
            num_rows,
            num_cols
        }
    }
}

impl Widget for FillingButtons {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        
        let row_constraints: Vec<Constraint> =
            (0..self.num_rows).map(|_| Constraint::Length(3)).collect(); // Each row 3 lines tall
        let row_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(row_constraints)
            .split(area);

        // Render buttons in the grid
        for i in 0..self.instructions.len() {
            let row = i / self.num_cols;
            let col = i % self.num_cols;

            // Split the current row into 2 columns
            let col_constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
            let col_layout = Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints(col_constraints)
                .split(row_layout[row]);

            // Render the button in the appropriate cell
            let button_block = Block::default().borders(Borders::ALL);
            let button_paragraph = Paragraph::new(self.instructions[i].to_owned()).block(button_block);
            button_paragraph.render(col_layout[col], buf);
        }
    }
}
