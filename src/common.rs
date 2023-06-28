

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Msg {
    AppClose,
    ButtonPressed(isize),
    Activate(Id),
    None,
}



#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Id {
    Label,
    MainMenu,
    LetterCounter,
    StockOverview
}
