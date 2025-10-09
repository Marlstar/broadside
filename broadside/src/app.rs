use freya::prelude::*;
use crate::board::Board;
use crate::board::SquareType;

use crate::menu::MainMenu;
use crate::screen::Screen;

pub static SCREEN: GlobalSignal<Screen> = Signal::global(|| Screen::Menu);

pub fn app() -> Element {
    let screen = SCREEN.resolve();
    let screen = &*screen.read();

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

            match screen {
                Screen::Menu => rsx!(MainMenu {}),
                Screen::Game => rsx!(Board { squares: board }),
                Screen::Settings => rsx!(label { text { "SETTINGS: TODO" } }),
            }
            // Board { squares: board }
        }
    }
}
