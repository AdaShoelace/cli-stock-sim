

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Msg {
    AppClose,
    BlurHeader,
    BlurStockOverview,
    ButtonPressed(isize),
    ChangeActivity(Id),
    None,
}



#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Id {
    Header,
    Label,
    LetterCounter,
    MainMenu,
    StockOverview
}
