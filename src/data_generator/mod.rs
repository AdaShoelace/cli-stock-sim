mod stock_data_generator;
mod mock_data_generator;

pub use stock_data_generator::StockDataGenerator;
pub use mock_data_generator::MockDataGenerator;



#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StockData {
    pub name: String,
    pub data: Vec<(f64, f64)>
}

impl Eq for StockData {}
