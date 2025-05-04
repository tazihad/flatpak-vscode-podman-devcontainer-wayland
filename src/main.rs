use iced::widget::button;
use iced::{Element, Task};

// State - holds the application status
#[derive(Default)]
struct FrozenRust {}

// Messages - represent user actions and events
#[derive(Debug, Clone)]
enum Message {
    ExitProgram,
}

// Update Logic - functionality that processes messages and updates state
fn update(_model: &mut FrozenRust, message: Message) -> Task<Message> {
    match message {
        Message::ExitProgram => iced::exit(),
    }
}

// View Logic - functionality that displays the state of the application
fn view(_model: &FrozenRust) -> Element<'_, Message> {
    button("Click to exit!")
        .on_press(Message::ExitProgram)
        .into()
}

fn main() -> iced::Result {
    iced::application("Hello cool world!", update, view).run()
}