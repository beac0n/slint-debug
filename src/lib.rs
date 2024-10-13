#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    // ... rest of your code ...
    slint::slint!{
        import { LineEdit, Button } from "std-widgets.slint";
        export component MainWindow inherits Window {
            VerticalLayout {
                LineEdit { text: "some not so long text"; }
                Button { text: "+"; width: 100%; }
            }
        }
    }
    MainWindow::new().unwrap().run().unwrap();
}