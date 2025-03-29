use avin::Instrument;
use avin::Manager;
use avin::MarketData;
use chrono::prelude::*;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let instr = Instrument::from("moex_share_sber").unwrap();
    let market_data = MarketData::BAR_M;
    let begin = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();

    let start = Instant::now();
    let bars = Manager::request(&instr, &market_data, &begin, &end).unwrap();
    let duration = start.elapsed();

    let first = &bars[0];
    let last = &bars[bars.len() - 1];

    dbg!(first.display());
    dbg!(last.display());
    dbg!(bars.len());

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
