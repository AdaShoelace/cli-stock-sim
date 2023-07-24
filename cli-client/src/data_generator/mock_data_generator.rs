// Crate imports
use super::StockData;
use crate::common::UserEvent;

// Std imports
use std::{collections::HashMap, sync::mpsc::Receiver};

// Third party imports
use rand::{thread_rng, Rng};
use tuirealm::{
    listener::{ListenerResult, Poll},
    Event,
};

pub struct MockDataGenerator {
    min: (f64, f64),
    max: (f64, f64),
    rx: Receiver<UserEvent>,
    data: HashMap<String, StockData>,
    current_stock: Option<String>
}

impl MockDataGenerator {
    pub fn new(min: (f64, f64), max: (f64, f64), rx: Receiver<UserEvent>) -> Self {
        Self {
            min,
            max,
            rx,
            data: {
                (0..100)
                    .map(|i| {
                        let name = format!("Stock_{i}");
                        (
                            name.clone(),
                            StockData {
                                name,
                                data: vec![],
                            },
                        )
                    })
                    .collect()
            },
            current_stock: None,
        }
    }

    pub fn generate(&mut self) {
        for stock_data in self.data.values_mut() {
            let y_max = self.max.1;
            let y_min = self.min.1;
            let x = stock_data.data.last().map(|x| x.0 + 1.0).unwrap_or(0.0);
            let y = Self::get_rand(y_min, y_max);
            stock_data.data.push((x, y));
        }
    }

    fn get_rand(min: f64, max: f64) -> f64 {
        let mut rng = thread_rng();
        let min = (min * 10.0) as usize;
        let max = (max * 10.0) as usize;
        rng.gen_range(min..max) as f64 / 10.0
    }
}

impl Poll<UserEvent> for MockDataGenerator {
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        self.generate();

        match self.rx.try_recv() {
            Ok(ev @ UserEvent::Init) => return Ok(Some(Event::User(ev))),
            Ok(UserEvent::CurrentStockChanged(stock)) => self.current_stock = Some(stock),
            _ => {}
        }

        let res = match self.current_stock {
            Some(ref current_stock) => {
                if let Some(stock_data) = self.data.get(current_stock) {
                    Some(Event::User(UserEvent::DataGenerated(stock_data.clone())))
                } else {
                    None
                }
            }
            None => None
        };
        Ok(res)
    }
}
