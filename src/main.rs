use gtk::gdk::Key;
use gtk::glib::Propagation;
use gtk::{
    prelude::*, EventControllerKey, EventControllerScroll, EventControllerScrollFlags, Label,
};
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "dev.nerdworks.Spacer";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let text = Label::builder()
        .label("Scroll to adjust transparency.")
        .opacity(1.0)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Spacer")
        .decorated(false)
        .opacity(0.8)
        .child(&text)
        .build();

    let scroll = EventControllerScroll::new(EventControllerScrollFlags::VERTICAL);
    let win = window.clone();
    scroll.connect_scroll(move |_, _, dy| {
        let opacity = if dy > 0.0 {
            win.opacity() + 0.1
        } else {
            win.opacity() - 0.1
        };

        let opacity = if opacity < 0.0 {
            0.0
        } else if opacity > 1.0 {
            1.0
        } else {
            opacity
        };

        win.set_opacity(opacity);
        Propagation::Stop
    });

    window.add_controller(scroll);

    let key = EventControllerKey::new();
    let win = window.clone();
    key.connect_key_pressed(move |_, key, _, _| {
        if key == Key::Escape {
            win.close();
        }
        Propagation::Stop
    });
    window.add_controller(key);

    window.present();
}
