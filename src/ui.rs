
slint::slint!{
    import { Button , HorizontalBox, VerticalBox} from "std-widgets.slint";
    export component App inherits Window {
        title: "相机";
        width: 800px;
        height: 600px;

        callback open-camera(int, int);
        
        in-out property <bool> camera-opened: false;
        in-out property <int> camera-width: 1280;
        in-out property <int> camera-height: 720;
        in property <image> video-frame <=> image.source;
        
        VerticalBox {
            HorizontalLayout {
                alignment: center;
                Rectangle {
                    border-color: gray;
                    border-width: 1px;
                    border-radius: 3px;
                    preferred-width: 100%;
                    height: 360px;
                    image := Image {
                        preferred-width: 100%;
                        height: 100%;
                    }
                }
            }
            HorizontalLayout {
                alignment: center;
                Button { 
                    text:  camera-opened? "关闭相机": "打开相机";
                    width: 100px;
                    height: 40px;
                    clicked => {
                        camera-opened = !camera-opened;
                        if(camera-opened){
                            open-camera(camera-width, camera-height)   
                        }
                    }
                }
            }
        }
    }
}
