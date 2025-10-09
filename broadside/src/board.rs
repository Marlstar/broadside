use freya::prelude::*;

#[component]
pub fn Board(squares: Vec<Vec<SquareType>>) -> Element {
    rsx!{
        for r in squares {
            Row { squares: r }
        }
    }
}

#[component]
pub fn Row(squares: Vec<SquareType>) -> Element {
    rsx!{
        rect {
            direction: "horizontal",

            for i in 0..squares.len() {
                Square { t: squares[i] }
            }
        }
    }
}

#[component]
pub fn Square(t: SquareType) -> Element {
    const SIZE: usize = 50;

    rsx!{
        rect {
            background: t.background(),
            width: "{SIZE}",
            height: "{SIZE}",
            corner_radius: "10",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SquareType {
    Unknown,
    Empty,
    Ship,
}
impl SquareType {
    pub fn background(&self) -> &str {
        match self {
            Self::Unknown => "#9399b2",
            Self::Empty => "#89b4fa",
            Self::Ship => "#1e1e2e"
        }
    }
}
