/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub const VERTICAL_WIDGET_SPACING: f64 = 20.0;
pub const TEXT_BOX_WIDTH: f64 = 200.0;
pub const WINDOW_TITLE: LString<crate::AppStore> = LString::new("Proton");

pub type LString<S> = druid::LocalizedString<S>;

pub enum WindowParams {
    Appellation { name: String },
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Constants;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
