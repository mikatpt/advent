use std::sync::Once;
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

static INIT: Once = Once::new();

fn install_tracing() {
    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

pub fn init() {
    INIT.call_once(|| {
        install_tracing();
        color_eyre::config::HookBuilder::default()
            .display_env_section(false)
            .install()
            .unwrap();
    })
}
