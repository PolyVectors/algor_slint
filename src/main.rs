slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.set_env_name(env!("CARGO_PKG_NAME").into());
    main_window.set_env_version(env!("CARGO_PKG_VERSION").into());
    main_window.set_env_description(env!("CARGO_PKG_DESCRIPTION").into());

    main_window.run()
}
