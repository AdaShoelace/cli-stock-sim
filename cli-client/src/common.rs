// Crate imports
use crate::data_generator::StockData;

pub mod constants {
    pub const GAME_NAME: &'static str = "Stock simulator";
    pub const ARROW_UP: char = '\u{2191}';
    pub const ARROW_DOWN: char = '\u{2193}';
    pub const DOT: char = '\u{00b7}';
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Msg {
    AppClose,
    BlurHeader,
    BlurStockList,
    BlurStockChart,
    BlurUsernameInput,
    BlurPasswordInput,
    ButtonPressed(isize),
    ChangeActivity(ActivityId),
    Login,
    StockDataChanged,
    UpdateStockChart(String),
    None,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Id {
    Header,
    Label,
    LetterCounter,
    Login,
    LoginPasswordInput,
    LoginUsernameInput,
    MainMenu,
    StockChart,
    StockList,
    TitleBar,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActivityId {
    Login,
    MainMenu,
    StockOverview,
    Profile
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum UserEvent {
    DataGenerated(StockData),
    CurrentStockChanged(String),
    TimeTick(i64),
    Init,
}

impl Eq for UserEvent {}

pub mod mock {
    use rand::Rng;
    use super::constants::{ARROW_UP, ARROW_DOWN};
    use crate::models::{Stock, Stocks};


    pub const INDUSTRIES: [&'static str; 4] = ["Tech", "Properties", "Pharmaceuticals", "Raw Materials"];

    pub fn generate_stocks(n: usize) -> Stocks {
        let mut rng = rand::thread_rng();
        (0..n)
            .map(|i| Stock {
                name: format!("Stock_{}", i),
                industry: {
                    let index = rng.gen_range(0usize..(INDUSTRIES.len() - 1));
                    INDUSTRIES[index].to_owned()
                },
                price: rng.gen_range(10..150),
            })
            .collect()
    }

    pub fn generate_stock_price_change() -> String {
        let mut rng = rand::thread_rng();
        let val: f32 = rng.gen_range(0.25..=2.5);

        let arrow = if rng.gen_bool(0.5) {
            ARROW_UP
        } else {
            ARROW_DOWN
        };
        format!("{val:.2} {}", arrow)
    }
}
