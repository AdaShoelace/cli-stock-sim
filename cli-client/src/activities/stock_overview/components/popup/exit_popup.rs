// Crate imports
use crate::common::{Msg, UserEvent};

// Third party imports
use tui_realm_stdlib::{Label, Phantom};
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{Alignment, BorderType, Borders, Color, Layout, Props},
    tui::layout::{Constraint, Direction, Rect},
    AttrValue, Attribute, Component, Event, Frame, MockComponent, State,
};

pub struct ExitPopUp {
    props: Props,
    children: Vec<Box<dyn MockComponent>>,
}

impl ExitPopUp {
    /*fn attr_child(&mut self, child: Children, attr: Attribute, value: AttrValue) {
        self.children[child as usize].attr(attr, value);
    }*/
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

impl Default for ExitPopUp {
    fn default() -> Self {
        let mut ret = Self {
            props: Props::default(),
            children: Default::default(),
        };
        ret = ret
            .borders(
                Borders::default()
                    .modifiers(BorderType::Rounded)
                    .color(Color::Yellow),
            )
            .layout(
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(&[
                        Constraint::Ratio(1, 4), // Padding
                        Constraint::Ratio(1, 4), // Label
                    ])
                    .margin(2),
            )
            .background(Color::Gray)
            .children(vec![
                Box::new(Phantom::default()),
                Box::new(
                    Label::default()
                        .alignment(Alignment::Center)
                        .foreground(Color::Yellow)
                        .text("Are you sure you want to quit?"),
                ),
            ]);
        ret
    }
}

impl MockComponent for ExitPopUp {
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
                // iter chunks
                for (i, chunk) in chunks.into_iter().enumerate() {
                    if let Some(child) = self.children.get_mut(i) {
                        child.view(render, chunk);
                    }
                }
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
        match cmd {
            _ => CmdResult::None,
        }
    }
}

impl Component<Msg, UserEvent> for ExitPopUp {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Cmd::Toggle,
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => Cmd::Delete,
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Cmd::Submit,
            Event::Keyboard(KeyEvent {
                code: Key::Char(c), ..
            }) => Cmd::Type(c),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };
        match self.perform(cmd) {
            CmdResult::Batch(_vec) => {
                // TODO: implement sending of credentials
                Some(Msg::None)
            }
            _ => Some(Msg::None),
        }
    }
}
