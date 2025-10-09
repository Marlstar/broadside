use freya::prelude::*;
use crate::{app::SCREEN, screen::Screen};

static SERVER_ADDR_INPUT: GlobalSignal<String> = Signal::global(String::new);

#[component]
pub fn MainMenu() -> Element {
    rsx!{
        ServerAddrInput {}
    }
}

#[component]
fn ServerAddrInput() -> Element {
    rsx!{
        rect { // Detect submit
            onkeydown: |e| {
                if e.key == Key::Enter {
                    println!("[main] connected to {SERVER_ADDR_INPUT}");
                    *SCREEN.write() = Screen::Game;
                }
            },
            Input {
                value: "{SERVER_ADDR_INPUT}",
                placeholder: "Server address",
                onchange: move |i| *SERVER_ADDR_INPUT.write() = i,
            }
        }
    }
}
