// Crate imports
use crate::common::{Msg, UserEvent};

// Third party imports
use tui_realm_stdlib::{Label, Phantom, Radio};
use tuirealm::{
    command::{self, Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{Alignment, BorderSides, BorderType, Borders, Color, Layout, Props},
    tui::layout::{Constraint, Direction, Rect, Layout as TuiLayout},
    tui::widgets::Clear,
    AttrValue, Attribute, Component, Event, Frame, MockComponent, State, StateValue
};

pub struct BuyPopUp {
    props: Props,
    children: Vec<Box<dyn MockComponent>>,
}

impl BuyPopUp {
    pub fn focus_radio(&mut self) {
        self.props.set(Attribute::Focus, AttrValue::Flag(false));
        self.children[2].attr(Attribute::Focus, AttrValue::Flag(true));
    }

    #[allow(dead_code)]
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }
    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    #[allow(dead_code)]
    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.attr(Attribute::Layout, AttrValue::Layout(layout));
        self
    }

    pub fn children(mut self, children: Vec<Box<dyn MockComponent>>) -> Self {
        self.children = children;
        self
    }
}

impl Default for BuyPopUp {
    fn default() -> Self {
        let mut ret = Self {
            props: Props::default(),
            children: Default::default(),
        };
        
        let main_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(&[
                        Constraint::Ratio(1, 4), // Padding
                        Constraint::Ratio(1, 4), // Label
                        Constraint::Ratio(1, 4), // Label
                    ]);

        ret = ret
            .borders(
                Borders::default()
                    .modifiers(BorderType::Rounded)
                    .color(Color::Yellow),
            )
            .layout(main_layout)
            .background(Color::Reset)
            .children(vec![
                Box::new(Phantom::default()),
                Box::new(
                    Label::default()
                        .alignment(Alignment::Center)
                        .background(Color::Reset)
                        .foreground(Color::Yellow)
                        .text("Buy this stock"),
                ),
                Box::new(
                    Radio::default()
                        .borders(Borders::default().sides(BorderSides::NONE))
                        .background(Color::Reset)
                        .foreground(Color::LightGreen)
                        .rewind(true)
                        .choices(&["No", "Yes"])
                        .value(0),
                ),
            ]);
        ret.focus_radio();
        ret
    }
}

impl MockComponent for BuyPopUp {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Make block
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let title = self.props.get(Attribute::Title).map(|x| x.unwrap_title());
            let div = crate::util::get_block(borders, title, true, None);
            // Render block
            render.render_widget(div, area);
            // Render children
            if let Some(layout) = self.props.get(Attribute::Layout).map(|x| x.unwrap_layout()) {
                // make chunks
                let chunks = layout.chunks(area);
                self.children[0].view(render, chunks[0]);
                self.children[1].view(render, chunks[1]);
                let radio_layout_x = TuiLayout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Ratio(1, 5),
                            Constraint::Ratio(1, 5),
                            Constraint::Ratio(1, 5),
                            Constraint::Ratio(1, 5),
                            Constraint::Ratio(1, 5),
                        ].as_ref()
                    ).split(chunks[2]);
                let radio_layout_y = TuiLayout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Ratio(1, 3),
                            Constraint::Ratio(1, 3),
                        ].as_ref()
                    ).split(radio_layout_x[2]);
                render.render_widget(Clear, radio_layout_y[0]);
                self.children[2].attr(Attribute::Background, AttrValue::Color(Color::Reset));
                self.children[2].view(render, radio_layout_y[0]);
            }
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        match attr {
            _ => {
                self.props.set(attr, value.clone());
                // Patch attribute to children
                self.children
                    .iter_mut()
                    .for_each(|x| x.attr(attr, value.clone()));
            }
        }
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.children[2].perform(cmd)
    }
}

impl Component<Msg, UserEvent> for BuyPopUp {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent { code: Key::Tab, .. })
            | Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => Cmd::Move(command::Direction::Right),
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => Cmd::Move(command::Direction::Left),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Cmd::Submit,
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::CloseBuyPopUp),
            _ => Cmd::None,
        };
        match self.perform(cmd) {
            CmdResult::Submit(State::One(StateValue::Usize(index))) => {
                match index {
                    0 => return Some(Msg::CloseBuyPopUp),
                    1 => return Some(Msg::AppClose),
                    _ => return Some(Msg::None)
                }
            }
            _ => Some(Msg::None),
        }
    }
}
