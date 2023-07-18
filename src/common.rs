// Crate imports
use crate::data_generator::StockData;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Msg {
    AppClose,
    BlurHeader,
    BlurStockOverview,
    BlurStockChart,
    ButtonPressed(isize),
    ChangeActivity(Id),
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
    MainMenu,
    StockOverview,
    StockChart,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum UserEvent {
    DataGenerated(StockData),
    CurrentStockChanged(String),
    Init,
}

impl Eq for UserEvent {}

pub mod mock {
    use rand::Rng;
    use crate::models::{Stock, Stocks};

    const ARROW_UP: char = '\u{2191}';
    const ARROW_DOWN: char = '\u{2193}';

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
