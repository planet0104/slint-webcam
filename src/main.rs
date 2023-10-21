use anyhow::Result;
mod ui;
mod camera;
use nokhwa::utils::Resolution;
use rfd::MessageDialog;
use ui::App;

use slint::ComponentHandle;

fn main() -> Result<()> {
    let app = App::new()?;

    let app_weak = app.as_weak();
    app.on_open_camera(move |width, height|{
        let app = app_weak.clone();
        println!("open-camera...");
        match camera::start(app, 0, Resolution::new(width as u32, height as u32)){
            Ok(()) => (),
            Err(err) => alert(&format!("{:?}", err))
        };
    });

    app.run()?;

    Ok(())
}

fn alert(msg: &str){
    let _ = MessageDialog::new()
    .set_title("提示")
    .set_description(msg)
    .show();
}
