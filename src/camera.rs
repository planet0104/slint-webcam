use std::{time::Duration, sync::{RwLock, Arc}};

use nokhwa::{utils::{CameraIndex, RequestedFormat, RequestedFormatType, Resolution}, pixel_format::RgbFormat, Camera};
use slint::{Weak, SharedPixelBuffer, Image};
use anyhow::{Result, anyhow};
use super::ui::App;

// 启动相机
pub fn start(app_weak: Weak<App>, camera_index: u32, resolution: Resolution) -> Result<()>{

    //测试摄像头是否打开成功
    let camera = open_camera(camera_index, resolution)?;
    drop(camera);

    //线程中启动15帧左右的摄像头
    std::thread::spawn(move ||{
        println!("{:?}", start_loop(app_weak, camera_index, resolution));
    });
    Ok(())
}

// 打开摄像头
fn open_camera(index: u32, resolution: Resolution) -> Result<Camera>{
    let index = CameraIndex::Index(index);
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::HighestResolution(resolution));
    let camera = Camera::new(index, requested)?;
    Ok(camera)
}

// 启动摄像头拍照线程
fn start_loop(app_weak: Weak<App>, camera_index: u32, resolution: Resolution) -> Result<()>{
    let mut camera = open_camera(camera_index, resolution)?;
    let opened = Arc::new(RwLock::new(true));
    loop{
        
        if let Ok(opened) = opened.read(){
            if !*opened{
                break;
            }
        }

        let frame = camera.frame()?;
        let decoded = frame.decode_image::<RgbFormat>()?;
        let shared = SharedPixelBuffer::clone_from_slice(&decoded, decoded.width(), decoded.height());
        let opened_clone = opened.clone();
        app_weak.upgrade_in_event_loop(move |app| {
            if let Ok(mut opened) = opened_clone.try_write(){
                *opened = app.get_camera_opened();
            }
            app.set_video_frame(Image::from_rgb8(shared))
        }).map_err(|err| anyhow!("{:?}", err))?;
       
        std::thread::sleep(Duration::from_millis(66));
    }
    Ok(())
}