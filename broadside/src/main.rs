use freya::prelude::{launch_cfg, LaunchConfig};
use broadside::app;

fn main() {
    let config = LaunchConfig::<()>::new()
        .with_title("Broadside")
        .with_size(700.0, 500.0)
        .with_decorations(true)
        .with_transparency(true)
        .with_background("transparent");

    launch_cfg(app, config)
}
