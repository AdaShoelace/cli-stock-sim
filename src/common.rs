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
    StockChart
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum UserEvent {
    DataGenerated(StockData),
    CurrentStockChanged(String),
    Init
}

impl Eq for UserEvent {}
