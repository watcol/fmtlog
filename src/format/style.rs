#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Style {
    pub fg: Option<colored::Color>,
    pub bg: Option<colored::Color>,
}
