// Crate imports
use crate::{common::{Id, Msg, UserEvent, mock::{generate_stocks}}, models::{Stocks, Stock}};

// Third party imports
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Attribute, AttrValue, Alignment, BorderType, Borders, Color, Style, TableBuilder, TextModifiers, TextSpan},
    Component, Event, MockComponent, State, StateValue,
};

use tui_realm_stdlib::Table;

#[derive(MockComponent)]
pub struct Overview {
    component: Table,
    stocks: Stocks,
}

impl Default for Overview {
    fn default() -> Self {
        let stocks = generate_stocks(100);
        let mut ret = Self {
            component: Table::default()
                .title("Companies".to_owned(), Alignment::Center)
                .background(Color::Reset)
                .borders(
                    Borders::default()
                    .color(Color::LightGreen)
                    .modifiers(BorderType::Rounded),
                )
                .foreground(Color::LightGreen)
                .modifiers(TextModifiers::BOLD)
                .scroll(true)
                .highlighted_color(Color::Yellow)
                .highlighted_str(">>")
                .rewind(true)
                .step(5)
                .row_height(1)
                .headers(&["Company", "Industry", "Price", "Owned", "Change"])
                .column_spacing(3)
                .widths(&[10, 20, 10, 10, 10])
                .table({
                    let mut tb = TableBuilder::default();
                    let len = stocks.len() - 1;
                    for (index, stock) in stocks.iter().enumerate() {
                        tb.add_col(TextSpan::from(stock.name.clone()))
                            .add_col(TextSpan::from(stock.industry.clone()))
                            .add_col(TextSpan::from(format!("{} $", stock.price)))
                            .add_col(TextSpan::from(format!("0")))
                            .add_col(TextSpan::from(format!("asd")));

                        if index != len {
                            tb.add_row();
                        }
                    }

                    tb.build()
                })
            .selected_line(0),
            stocks,
        };

        let stock_name = if let Some(stock) = ret.stocks.get(0) {
            stock.name.clone()
        } else {
            String::new()
        };

        ret.attr(Attribute::Custom("CurrentStockName"), AttrValue::String(stock_name));
        ret
    }
}

impl Component<Msg, UserEvent> for Overview {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let cmd_res = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::NONE,
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('j'),
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::NONE,
            })

            | Event::Keyboard(KeyEvent {
                code: Key::Char('k'),
                modifiers: KeyModifiers::NONE,
            })
            => self.perform(Cmd::Move(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::SHIFT,
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::SHIFT,
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::BlurStockOverview),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. })
                | Event::Keyboard(KeyEvent {
                    code: Key::Backspace,
                    ..
                }) => return Some(Msg::ChangeActivity(Id::MainMenu)),
            Event::User(UserEvent::Init) => {
                if let Some(stock) = self.stocks.get(0) {
                    return Some(Msg::UpdateStockChart(stock.name.clone()))
                } else {
                    CmdResult::None
                }
            }
                _ => CmdResult::None,
        };

        match cmd_res {
            CmdResult::Changed(State::One(StateValue::Usize(index))) => {
                if let Some(stock) = self.stocks.get(index) {
                    let ret = Some(Msg::UpdateStockChart(stock.name.clone()));
                    self.attr(Attribute::Custom("CurrentStockName"), AttrValue::String(stock.name.clone()));
                    return ret;
                }
            }
            _ => {}
        }
        Some(Msg::None)
    }
}
