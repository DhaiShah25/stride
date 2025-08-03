use std::time::{Duration, Instant};

use humantime::Duration as hDuration;

use iced::widget::{button, column, container, text};
use iced::{Alignment, Element, Event, Length, Task, Theme, event};
use iced_layershell::Application;
use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::to_layer_message;

use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    message: String,

    #[arg(default_value = "20s")]
    duration: hDuration,
}

pub fn main() -> Result<(), iced_layershell::Error> {
    let binded_output_name = std::env::args().nth(1);

    let start_mode = match binded_output_name {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };

    Wellness::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((1920, 1080)),

            anchor: Anchor::Bottom | Anchor::Left | Anchor::Right | Anchor::Top,

            layer: Layer::Overlay,
            start_mode,
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::None,
            ..Default::default()
        },
        flags: dbg!(Args::parse()),
        ..Default::default()
    })
}

struct Wellness {
    message: String,
    duration: Duration,
    start: Instant,
}

#[to_layer_message]
#[derive(Debug, Clone)]
#[doc = "Some docs"]
enum Message {
    Kill,
    Tick(Instant),
}

impl Application for Wellness {
    type Message = Message;
    type Flags = Args;
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(flags: Self::Flags) -> (Self, Task<Message>) {
        (
            Self {
                message: flags.message,
                duration: *flags.duration,
                start: Instant::now(),
            },
            Task::done(Message::AnchorSizeChange(
                Anchor::Bottom | Anchor::Left | Anchor::Right | Anchor::Top,
                (0, 0),
            )),
        )
    }

    fn namespace(&self) -> String {
        String::from("Hello - Iced")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(self.duration).map(Message::Tick)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Kill => iced::exit(),
            Message::Tick(now) => {
                println!("{:?}", &now);
                if now.duration_since(self.start.into()) >= self.duration {
                    Task::done(Message::Kill)
                } else {
                    Task::none()
                }
            }

            _ => unreachable!(),
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                text(&self.message).size(100),
                button("Skip")
                    .on_press(Message::Kill)
                    .padding(10)
                    .style(|_: &Theme, _| {
                        let theme = &Theme::Nord;
                        let palette = theme.extended_palette();

                        button::Style::default().with_background(palette.primary.base.color)
                    })
            ]
            .spacing(20)
            .align_x(Alignment::Center)
            .height(Length::Shrink),
        )
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .padding(20)
        .center_x(Length::Fill)
        .center_x(Length::Fill)
        .into()
    }

    fn style(&self, _: &Self::Theme) -> iced_layershell::Appearance {
        use iced_layershell::Appearance;

        let theme = Theme::Nord;

        Appearance {
            background_color: theme.palette().background.scale_alpha(0.8),
            text_color: theme.palette().text,
        }
    }
}
