// Crate imports
use crate::common::{Msg, UserEvent};

// Third party imports
use tui_realm_stdlib::Chart;
use tuirealm::{
    command::CmdResult,
    props::{
        Alignment, AttrValue, Attribute, BorderType, Borders, Color, Dataset, PropPayload,
        PropValue, Style,
    },
    tui::{symbols::Marker, widgets::GraphType},
    Component, Event, MockComponent,
};

const X_BOUND_MIN: f64 = 0.0;
const X_BOUND_MAX: f64 = 50.0;
const Y_BOUND_MIN: f64 = 0.0;
const Y_BOUND_MAX: f64 = 50.0;

#[derive(MockComponent)]
pub struct StockChart {
    component: Chart,
}

impl StockChart {
    pub fn new() -> Self {
        Self {
            component: Chart::default()
                .disabled(true)
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Double)
                        .color(Color::Yellow),
                )
                .x_style(Style::default().fg(Color::LightBlue))
                .x_title("Time")
                .x_bounds((X_BOUND_MIN, X_BOUND_MAX))
                .x_labels(&["1Y", "10M", "8M", "6M", "4M", "2M", "now"])
                .y_style(Style::default().fg(Color::Yellow))
                .y_title("Price")
                .y_bounds((Y_BOUND_MIN, Y_BOUND_MAX))
                .y_labels(&[
                    "0", "5", "10", "15", "20", "25", "30", "35", "40", "45", "50",
                ]),
        }
    }
}

impl Component<Msg, UserEvent> for StockChart {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::User(UserEvent::DataGenerated(stock_data)) => {
                // Update data
                self.attr(
                    Attribute::Title,
                    AttrValue::Title((stock_data.name.clone(), Alignment::Left)),
                );
                let dataset = Dataset::default()
                    .name(stock_data.name)
                    .graph_type(GraphType::Line)
                    .marker(Marker::Braille)
                    .style(Style::default().fg(Color::Cyan))
                    .data(stock_data.data);
                self.attr(
                    Attribute::Dataset,
                    AttrValue::Payload(PropPayload::Vec(vec![PropValue::Dataset(dataset)])),
                );
                CmdResult::None
            }
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
