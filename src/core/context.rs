/*
    Appellation: controller <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WindowParams {
    pub shape: (f64, f64),
}

impl WindowParams {
    pub fn new(shape: (f64, f64)) -> Result<Self, scsys::BoxError> {
        Ok(Self { shape })
    }
    pub fn default() -> Self {
        Self::new((800f64, 600f64)).ok().unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Context {
    pub name: String,
    pub pages: Vec<String>,
    pub win: WindowParams,
}

impl Context {
    pub fn new(
        name: String,
        pages: Vec<String>,
        win: WindowParams,
    ) -> Result<Self, scsys::BoxError> {
        Ok(Self {
            name,
            pages,
            win,
        })
    }
    pub fn default() -> Self {
        let pages: Vec<String> = vec![
            "Dashboard",
            "Account",
            "Connect",
            "Discover",
            "Create",
            "Settings",
        ]
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>();
        Self::new("Proton".to_string(), pages.clone(), WindowParams::default())
            .ok()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
