/*
    Appellation: faces <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        A face is equivalent to a webpage, route, etc
*/
use druid::WidgetExt;
pub use home::*;

pub mod calc;
mod home;

pub trait Face<Cmp: druid::Data> {
    fn appellation(&self) -> String
        where
            Self: Sized;
    fn display(&self);
    fn layout(&self);
}

#[derive(Clone, Debug)]
pub struct Page {
    pub id: scsys::Id,
    pub shape: (f64, f64),
    pub title: druid::LocalizedString<Space>,
}

impl Page {
    pub fn default() -> Result<Self, scsys::BoxError> {
        Ok(Self {
            id: scsys::Id::Obj(bson::oid::ObjectId::new()),
            shape: (800f64, 600f64),
            title: druid::LocalizedString::<Space>::new("Proton"),
        })
    }
}

#[derive(
Clone, Debug, Hash, PartialEq, druid::Data, druid::Lens, serde::Deserialize, serde::Serialize,
)]
pub struct SubSpace {
    pub username: String,
    pub password: String,
}

impl SubSpace {
    fn authorize(&self) -> Result<bool, scsys::BoxError> {
        todo!()
    }

    fn constructor(username: String, password: String) -> Result<Self, scsys::BoxError> {
        Ok(Self { username, password })
    }

    pub fn new(username: String, password: String) -> Self {
        match Self::constructor(username, password) {
            Ok(v) => v,
            Err(e) => panic!("SubSpace Error: {}", e),
        }
    }
    pub fn from(username: &str, password: &str) -> Self {
        Self::new(username.to_string(), password.to_string())
    }

    pub fn init() -> Self {
        Self::new("".to_string(), "".to_string())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, druid::Data, druid::Lens)]
pub struct Space {
    pub subspace: SubSpace,
    pub timestamp: i64,
}

impl Space {
    fn constructor(subspace: SubSpace, timestamp: i64) -> Result<Self, scsys::BoxError> {
        Ok(Self {
            subspace,
            timestamp,
        })
    }

    pub fn new(subspace: SubSpace, timestamp: i64) -> Self {
        match Self::constructor(subspace, timestamp) {
            Ok(v) => v,
            Err(e) => panic!("Space Error: {}", e),
        }
    }

    pub fn init() -> Self {
        Self::new(SubSpace::init(), chrono::Utc::now().timestamp())
    }

    pub fn canvas() -> druid::widget::Flex<Self> {
        let gradient = druid::LinearGradient::new(
            druid::UnitPoint::TOP_LEFT,
            druid::UnitPoint::BOTTOM_RIGHT,
            (crate::theme::DARKER_GREY, crate::theme::LIGHTER_GREY),
        );
        let label = druid::widget::Label::new(|data: &Self, _env: &druid::Env| {
            format!(
                "Welcome to Proton\nSession Started at {}",
                chrono::Utc::now()
            )
        });

        let layout = druid::widget::Flex::column()
            .with_flex_child(
                druid::widget::Flex::row().with_flex_child(
                    druid::widget::Label::new("top left")
                        .center()
                        .border(crate::theme::DARK_GREY, 2.0)
                        .padding(3.0),
                    1.0,
                ),
                1.0,
            )
            .with_child(label)
            .with_spacer(crate::VERTICAL_WIDGET_SPACING);

        layout
    }

    pub fn display() -> impl druid::Widget<Self> {
        druid::widget::Align::centered(Self::canvas())
    }
}
