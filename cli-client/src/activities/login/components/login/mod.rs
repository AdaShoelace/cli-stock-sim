mod password_input;
mod username_input;

// Crate imports
use crate::common::Msg;
use password_input::PasswordInput;
use username_input::UsernameInput;

// Third party imports
use tui_realm_stdlib::{Label, Phantom};
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, NoUserEvent},
    props::{Alignment, BorderType, Borders, Color, Layout, Props},
    tui::layout::{Constraint, Direction, Rect},
    AttrValue, Attribute, Component, Event, Frame, MockComponent, State,
};

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Children {
    UsernameInput = 2,
    PasswordInput = 3,
}

pub struct Login {
    props: Props,
    children: Vec<Box<dyn MockComponent>>,
    focused_child: Children,
}

impl Login {
    fn attr_child(&mut self, child: Children, attr: Attribute, value: AttrValue) {
        self.children[child as usize].attr(attr, value);
    }

    fn focus_username(&mut self) {
        // Blur password
        self.attr_child(Children::PasswordInput, Attribute::Focus, AttrValue::Flag(false));

        // Focus new
        self.attr_child(Children::UsernameInput, Attribute::Focus, AttrValue::Flag(true));
        //
        // Set new focused_child
        self.focused_child = Children::UsernameInput;
    }

    fn focus_password(&mut self) {
        // Blur username
        self.attr_child(Children::UsernameInput, Attribute::Focus, AttrValue::Flag(false));

        // Focus new
        self.attr_child(Children::PasswordInput, Attribute::Focus, AttrValue::Flag(true));
        //
        // Set new focused_child
        self.focused_child = Children::PasswordInput;

    }
    fn toggle_focus(&mut self) {
        match self.focused_child {
            Children::UsernameInput => self.focus_password(),
            Children::PasswordInput => self.focus_username(),
        }
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

impl Default for Login {
    fn default() -> Self {
        let mut ret = Self {
            props: Props::default(),
            children: Default::default(),
            focused_child: Children::UsernameInput,
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
                        Constraint::Ratio(1, 4), // Username input
                        Constraint::Ratio(1, 4), // Password input
                    ])
                    .margin(2),
            )
            .children(vec![
                Box::new(Phantom::default()),
                Box::new(
                    Label::default()
                        .alignment(Alignment::Center)
                        .foreground(Color::Yellow)
                        .text("Enter username and password to log in"),
                ),
                Box::new(UsernameInput::default()),
                Box::new(PasswordInput::default()),
            ]);
        ret.attr(Attribute::Focus, AttrValue::Flag(false));
        ret.attr_child(ret.focused_child, Attribute::Focus, AttrValue::Flag(true));
        ret.attr_child(
            Children::PasswordInput,
            Attribute::Focus,
            AttrValue::Flag(false),
        );
        ret
    }
}

impl MockComponent for Login {
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
            a @ Attribute::Focus => self.attr_child(self.focused_child, a, value),
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
            Cmd::Toggle => {
                self.toggle_focus();
                CmdResult::None
            }
            Cmd::Submit => match self.focused_child {
                Children::UsernameInput => {
                    self.focus_password();
                    CmdResult::None
                }
                Children::PasswordInput => {
                    let username_state = self.children[Children::UsernameInput as usize].state();
                    let password_state = self.children[Children::UsernameInput as usize].state(); 
                    CmdResult::Batch(vec![CmdResult::Submit(username_state), CmdResult::Submit(password_state)])
                }
            }
            x => self.children[self.focused_child as usize].perform(x),
        }
    }
}

impl Component<Msg, NoUserEvent> for Login {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Cmd::Toggle,
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => Cmd::Delete,
            Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => Cmd::Submit,
            Event::Keyboard(KeyEvent {
                code: Key::Char(c), ..
            }) => Cmd::Type(c),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };
        match self.perform(cmd) {
            CmdResult::Batch(_vec) => {
                // TODO: implement sending of credentials
                Some(Msg::Login)
            }
            _ => Some(Msg::None),
        }
    }
}
