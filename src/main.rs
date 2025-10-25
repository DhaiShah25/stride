use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, glib};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;

const VISIBLE_SECS: u64 = 20;
const HIDDEN_SECS: u64 = 1200;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(build_ui);

    application.run()
}

fn build_ui(app: &Application) {
    let provider = gtk4::CssProvider::new();
    let css_data = r#"
        .title-label {
            font-size: 192pt;
            font-weight: bold;
            color: #4CAF50;
            font-family: "MonaspiceKr NFP", "monospace";
        }

        .transparent-window {
            background-color: rgba(42, 39, 63, 0.7);
        }
    "#;
    provider.load_from_data(css_data);

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My Wayland Panel")
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_css_classes(&["transparent-window"]);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_keyboard_mode(KeyboardMode::None);

    let text = gtk4::Label::new(Some("Eye Break"));
    text.set_css_classes(&["title-label"]);

    let button = Button::with_label("Click me!");
    button.connect_clicked(|_| {
        eprintln!("Clicked!");
    });
    window.set_child(Some(&text));

    show_and_schedule_hide(&window);
}

fn show_and_schedule_hide(window: &gtk4::ApplicationWindow) {
    window.set_visible(true);

    let window = window.clone();
    glib::timeout_add_local(Duration::from_secs(VISIBLE_SECS), move || {
        hide_and_schedule_show(&window);
        glib::ControlFlow::Break
    });
}

fn hide_and_schedule_show(window: &gtk4::ApplicationWindow) {
    window.set_visible(false);

    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs(1))
        .amplify(0.20);
    sink.append(source);

    sink.sleep_until_end();

    let window = window.clone();
    glib::timeout_add_local(Duration::from_secs(HIDDEN_SECS - 1), move || {
        window.set_visible(true);
        show_and_schedule_hide(&window);
        glib::ControlFlow::Break
    });
}
