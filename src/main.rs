slint::include_modules!();

const MVA: f64 = 0.24;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_delt_sum(move |string| {
        let ui = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let mva: f64 = num * MVA;
        let result = format!("MVA: {:?}", mva);
        ui.set_resultat(result.into());
    });

    ui.run()
}
