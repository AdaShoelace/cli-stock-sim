// Crate imports
use crate::common::{Id, Msg};

// Third party imports
use rand::Rng;
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, BorderType, Borders, Color, TableBuilder, TextModifiers, TextSpan, Style},
    Component, Event, MockComponent, NoUserEvent,
};

use tui_realm_stdlib::Table;

const INDUSTRIES: [&'static str; 4] = ["Tech", "Properties", "Pharmaceuticals", "Raw Materials"];

struct Stock {
    name: String,
    industry: String,
    price: usize,
}

type Stocks = Vec<Stock>;

fn generate_stocks(n: usize) -> Stocks {
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

#[derive(MockComponent)]
pub struct Overview {
    component: Table,
}

impl Default for Overview {
    fn default() -> Self {
        let stocks = generate_stocks(100);
        Self {
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
                .headers(&["Company", "Industry", "Price"])
                .column_spacing(3)
                .widths(&[30, 20, 50])
                .table({
                    let mut tb = TableBuilder::default();
                    let len = stocks.len();
                    for (index, stock) in stocks.into_iter().enumerate() {
                        tb.add_col(TextSpan::from(stock.name))
                            .add_col(TextSpan::from(stock.industry))
                            .add_col(TextSpan::from(format!("{} $", stock.price)));

                        if index != len {
                            tb.add_row();
                        }
                    }

                    tb.build()
                })
                .selected_line(0),
        }
    }
}

impl Component<Msg, NoUserEvent> for Overview {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Up)),
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
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
