use freya::prelude::*;
use crate::board::Board;
use crate::board::SquareType;

pub fn app() -> Element {
    let board = {
        use SquareType::*;
        vec![
            vec![Unknown, Unknown, Empty, Ship],
            vec![Empty, Unknown, Empty, Ship],
            vec![Ship, Empty, Empty, Ship],
            vec![Ship, Empty, Empty, Empty],
        ]
    };

    rsx!{
        rect {
            width: "fill",
            height: "fill",
            main_align: "center",
            cross_align: "center",
            background: "transparent",

            Board { squares: board }
        }
    }
}
