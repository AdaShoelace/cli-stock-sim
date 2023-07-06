// Crate imports
use crate::{
    common::{Id, Msg},
    components::{LetterCounter, MainMenu, stock_market},
};

// Std imports
use std::time::Duration;

// Third party imports
use tuirealm::{
    props::Alignment,
    terminal::TerminalBridge,
    tui::layout::{Constraint, Direction, Layout},
    Application, AttrValue, Attribute, EventListenerCfg, NoUserEvent, Update,
};
pub struct Model {
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
    current_component: Id
}

impl Default for Model {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
            current_component: Id::MainMenu
        }
    }
}

impl Model {
    pub fn view(&mut self) {
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Length(3)].as_ref())
                    .split(f.size());
                self.app.view(&self.current_component, f, chunks[0]);
            })
            .is_ok());
    }

    fn init_app() -> Application<Id, Msg, NoUserEvent> {
        // Setup application
        // NOTE: NoUserEvent is a shorthand to tell tui-realm we're not going to use any custom user event
        // NOTE: the event listener is configured to use the default crossterm input listener and to raise a Tick event each second
        // which we will use to update the clock
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        assert!(app
            .mount(
                Id::MainMenu,
                Box::new(MainMenu::default()),
                Vec::default()
            )
            .is_ok());
        assert!(app
            .mount(
                Id::StockOverview,
                Box::new(stock_market::Overview::default()),
                Vec::default()
            )
            .is_ok());

        assert!(app.active(&Id::MainMenu).is_ok());
        app
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            self.redraw = true;

            match msg {
                Msg::AppClose => {
                    self.quit = true;
                    None
                }
                Msg::ButtonPressed(c) => {
                    assert!(self
                        .app
                        .attr(
                            &Id::LetterCounter,
                            Attribute::Title,
                            AttrValue::Title((format!("Button pressed: {}", c), Alignment::Center))
                        )
                        .is_ok());
                    None
                }
                Msg::ChangeActivity(view) => {
                    self.current_component = view;
                    assert!(self.app.active(&view).is_ok());
                    None
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
