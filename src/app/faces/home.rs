/*
    Appellation: home <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use druid::WidgetExt;
use druid::{
    widget::{Button, Flex, Label, Split, TextBox, ViewSwitcher},
    Env,
};

#[derive(Clone, Debug, druid::Data, druid::Lens)]
pub struct HomeSpace {
    pub authenticated: bool,
    pub current_view: u32,
    pub current_text: String,
}

impl HomeSpace {
    fn constructor(
        authenticated: bool,
        current_view: u32,
        current_text: String,
    ) -> Result<Self, scsys::BoxError> {
        Ok(Self {
            authenticated,
            current_view,
            current_text,
        })
    }
    pub fn display() -> impl druid::Widget<Self> {
        druid::widget::Align::centered(create_container_homepage())
    }

    pub fn init() -> Self {
        Self::new(false, 0, String::from(""))
    }
    pub fn new(authenticated: bool, current_view: u32, current_text: String) -> Self {
        match Self::constructor(authenticated, current_view, current_text) {
            Ok(v) => v,
            Err(e) => panic!("App State Error: {}", e),
        }
    }
}

pub fn create_container_homepage() -> Flex<HomeSpace> {
    let mut controller = Flex::column();
    controller.add_child(
        Label::new(|data: &u32, _env: &Env| format!("Current view: {}", data))
            .lens(HomeSpace::current_view),
    );
    for i in 0..6 {
        controller.add_spacer(80.);
        controller.add_child(
            Button::new(format!("View {}", i))
                .on_click(move |_event, data: &mut u32, _env| {
                    *data = i;
                })
                .lens(HomeSpace::current_view),
        );
    }

    let view_switcher = ViewSwitcher::new(
        |data: &HomeSpace, _env| data.current_view,
        |selector, _data, _env| match selector {
            0 => Box::new(Label::new("Simple Label").center()),
            1 => Box::new(
                Button::new("Simple Button").on_click(|_event, _data, _env| {
                    println!("Simple button clicked!");
                }),
            ),
            2 => Box::new(
                Button::new("Another Simple Button").on_click(|_event, _data, _env| {
                    println!("Another simple button clicked!");
                }),
            ),
            3 => Box::new(
                Flex::column()
                    .with_flex_child(Label::new("Here is a label").center(), 1.0)
                    .with_flex_child(
                        Button::new("Button").on_click(|_event, _data, _env| {
                            println!("Complex button clicked!");
                        }),
                        1.0,
                    )
                    .with_flex_child(TextBox::new().lens(HomeSpace::current_text), 1.0)
                    .with_flex_child(
                        Label::new(|data: &String, _env: &Env| format!("Value entered: {}", data))
                            .lens(HomeSpace::current_text),
                        1.0,
                    ),
            ),
            4 => Box::new(
                Split::columns(
                    Label::new("Left split").center(),
                    Label::new("Right split").center(),
                )
                    .draggable(true),
            ),
            _ => Box::new(Label::new("Unknown").center()),
        },
    );

    Flex::row()
        .with_child(controller)
        .with_flex_child(view_switcher, 1.0)
}
