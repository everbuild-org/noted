pub struct Panes {
    pub files: bool,
    pub graph: bool,
}

impl Default for Panes {
    fn default() -> Self {
        Self {
            files: false,
            graph: false,
        }
    }
}

pub enum PaneToggle {
    Files(bool),
    Graph(bool),
}
