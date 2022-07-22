/*
    Appellation: navbar <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::ApplicationState;
use druid::{
    widget::{Button, Flex, Label},
    WidgetExt,
};

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Navbar;

impl Navbar {
    pub fn new(controller: crate::Controller) -> Result<Flex<ApplicationState>, scsys::BoxError> {
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
        Ok(component)
    }
}
