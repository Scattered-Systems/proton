/*
    Appellation: index <module>
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
pub struct CurrentState {
    pub angle: String,
    pub message: String,
    pub query: String,
    pub view: String,
}

#[derive(Clone, Debug, druid::Data, druid::Lens)]
pub struct EditorState {
    pub message_id: String,
    pub guest: String,
    pub data: String,
}

#[derive(Clone, Debug, druid::Data, druid::Lens)]
pub struct ApplicationState {
    pub message: String,
    pub query: String,
    pub view: u32,
}

impl ApplicationState {
    pub fn display() -> impl druid::Widget<Self> {
        druid::widget::Align::centered(create_application_canvas())
    }
    pub fn init() -> Self {
        Self::new(String::from(""), String::from(""), 0u32)
            .ok()
            .unwrap()
    }
    pub fn new(message: String, query: String, view: u32) -> Result<Self, scsys::BoxError> {
        Ok(Self {
            message,
            query,
            view,
        })
    }
}

pub fn create_navbar(
    controller: crate::Controller,
) -> Result<Flex<ApplicationState>, scsys::BoxError> {
    let pages: [&str; 6] = [
        "Dashboard",
        "Artifacts",
        "Connect",
        "Discover",
        "Create",
        "Settings",
    ];
    let mut component = Flex::row();
    component.add_child(Label::new(&*controller.name).center());
    for i in 0..6 {
        component.add_spacer(25.0);
        component.add_child(
            Button::new(format!("{}", pages[i]))
                .on_click(move |_event, data: &mut u32, _env| {
                    *data = i.try_into().ok().unwrap();
                })
                .lens(ApplicationState::view),
        );
    }
    Ok(component)
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Navbar;

impl Navbar {
    pub fn component(controller: crate::Controller) -> Flex<ApplicationState> {
        let mut component = Flex::row();
        component.add_child(Label::new(&*controller.name).center());
        for i in 0..6 {
            component.add_spacer(25.0);
            component.add_child(
                Button::new(format!("{}", controller.pages.clone()[i]))
                    .on_click(move |_event, data: &mut u32, _env| {
                        *data = i.try_into().ok().unwrap();
                    })
                    .lens(ApplicationState::view),
            );
        }
        component
    }
    pub fn constructor() -> crate::Controller {
        crate::Controller::default()
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Views;

impl Views {
    pub fn constructor() -> ViewSwitcher<ApplicationState, u32> {
        ViewSwitcher::new(
            |data: &ApplicationState, _env| data.view,
            |selector, _data, _env| match selector {
                0 => Box::new(Self::default_page()),
                1 => Box::new(Self::account_page()),
                2 => Box::new(Self::connection_board()),
                3 => Box::new(Self::discovery_page()),
                4 => Box::new(Self::creation_hub()),
                _ => Box::new(Self::control_panel()),
            },
        )
    }

    /// Describes the default view for our application
    pub fn default_page() -> Flex<ApplicationState> {
        Flex::column().with_flex_child(
            Flex::row()
                .with_flex_child(Label::new("Sidebar").center(), 0.75)
                .with_flex_child(Label::new("Display").center(), 3.0)
                .with_flex_child(Label::new("Feed").center(), 0.75),
            1.0,
        )
    }

    /// Describes the account view (index: 1) for our application
    pub fn account_page() -> Flex<ApplicationState> {
        Flex::column().with_flex_child(
            Flex::row().with_flex_child(Label::new("ENS").center(), 1.0),
            1.0,
        )
    }

    /// Describes
    pub fn connection_board() -> Split<ApplicationState> {
        let stream: Vec<&str> = vec!["AppFeed", "Person"];
        println!("{:#?}", stream.clone());
        let feed = Flex::column().with_flex_child(Label::new("Feed").center().expand(), 1.0);
        let editor = Flex::column()
            .with_flex_child(Label::new("Message").center().expand(), 0.25)
            .with_flex_child(Label::new("History").center().expand(), 1.0)
            .with_flex_child(TextBox::new().lens(ApplicationState::message), 0.25);
        Split::columns(feed, editor).draggable(true)
    }

    /// Combining a block explorer, global marketplace, and search engine into a single portal
    pub fn discovery_page() -> Flex<ApplicationState> {
        Flex::column()
            .with_flex_child(Label::new("Here is a label").center(), 1.0)
            .with_flex_child(
                Button::new("Button").on_click(|_event, _data, _env| {
                    println!("Complex button clicked!");
                }),
                1.0,
            )
            .with_flex_child(TextBox::new().lens(ApplicationState::query), 1.0)
            .with_flex_child(
                Label::new(|data: &String, _env: &Env| format!("Value entered: {}", data))
                    .lens(ApplicationState::query),
                1.0,
            )
    }

    ///
    pub fn creation_hub() -> Flex<ApplicationState> {
        Flex::column()
            .with_flex_child(Label::new("Here is a label").center(), 1.0)
            .with_flex_child(
                Button::new("Button").on_click(|_event, _data, _env| {
                    println!("Complex button clicked!");
                }),
                1.0,
            )
            .with_flex_child(TextBox::new().lens(ApplicationState::query), 1.0)
            .with_flex_child(
                Label::new(|data: &String, _env: &Env| format!("Value entered: {}", data))
                    .lens(ApplicationState::query),
                1.0,
            )
    }

    ///
    pub fn control_panel() -> Flex<ApplicationState> {
        Flex::column()
            .with_flex_child(
                Flex::row().with_flex_child(Label::new("Account").center().expand(), 1.0),
                1.0,
            )
            .with_flex_spacer(1.0)
            .with_flex_child(
                Flex::row().with_flex_child(Label::new("Contacts").center().expand(), 1.0),
                1.0,
            )
            .with_flex_child(
                Flex::row().with_flex_child(Label::new("Devices").center().expand(), 1.0),
                1.0,
            )
    }
}

pub fn create_application_canvas() -> Flex<ApplicationState> {
    let controller = crate::Controller::default();
    Flex::column()
        .with_child(create_navbar(controller).ok().unwrap())
        .with_flex_child(Views::constructor(), 1.0)
}
