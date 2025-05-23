/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::direction::Direction;
use crate::core::operation::Operation;
use crate::core::transaction::Transaction;
use bitcode::{Decode, Encode};

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub enum MarketOrder {
    New(NewMarketOrder),
    Posted(PostedMarketOrder),
    Filled(FilledMarketOrder),
    Rejected(RejectedMarketOrder),
}
impl MarketOrder {
    pub fn new(direction: Direction, lots: u32) -> NewMarketOrder {
        NewMarketOrder { direction, lots }
    }
}
impl std::fmt::Display for MarketOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::New(order) => write!(f, "{order}"),
            Self::Posted(order) => write!(f, "{order}"),
            Self::Filled(order) => write!(f, "{order}"),
            Self::Rejected(order) => write!(f, "{order}"),
        }
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct NewMarketOrder {
    pub direction: Direction,
    pub lots: u32,
}
impl NewMarketOrder {
    pub fn post(self, broker_id: &str) -> PostedMarketOrder {
        PostedMarketOrder {
            direction: self.direction,
            lots: self.lots,
            broker_id: broker_id.to_string(),
            transactions: Vec::new(),
        }
    }
    pub fn reject(self, meta: &str) -> RejectedMarketOrder {
        RejectedMarketOrder {
            direction: self.direction,
            lots: self.lots,
            meta: meta.to_string(),
        }
    }
}
impl std::fmt::Display for NewMarketOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MarketOrder::New={} {}", self.direction, self.lots)
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct PostedMarketOrder {
    pub direction: Direction,
    pub lots: u32,
    pub broker_id: String,
    pub transactions: Vec<Transaction>,
}
impl PostedMarketOrder {
    pub fn add_transaction(&mut self, t: Transaction) {
        self.transactions.push(t);
    }
    pub fn fill(self, ts_nanos: i64, commission: f64) -> FilledMarketOrder {
        let operation =
            Operation::from(ts_nanos, &self.transactions, commission);
        FilledMarketOrder {
            direction: self.direction,
            lots: self.lots,
            broker_id: self.broker_id,
            transactions: self.transactions,
            operation,
        }
    }
}
impl std::fmt::Display for PostedMarketOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MarketOrder::Posted={} {} id={} t={:?}",
            self.direction, self.lots, self.broker_id, self.transactions
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct FilledMarketOrder {
    pub direction: Direction,
    pub lots: u32,
    pub broker_id: String,
    pub transactions: Vec<Transaction>,
    pub operation: Operation,
}
impl std::fmt::Display for FilledMarketOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MarketOrder::Filled={} {} id={} t={:?} {}",
            self.direction,
            self.lots,
            self.broker_id,
            self.transactions,
            self.operation
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct RejectedMarketOrder {
    pub direction: Direction,
    pub lots: u32,
    pub meta: String,
}
impl std::fmt::Display for RejectedMarketOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MarketOrder::Rejected={} {} meta={}",
            self.direction, self.lots, self.meta
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn new_post_fill() {
        let new = MarketOrder::new(Direction::Buy, 10);

        let mut posted = new.post("order_id=100500");
        assert_eq!(posted.broker_id, "order_id=100500");

        let t1 = Transaction::new(5, 320.0);
        posted.add_transaction(t1);
        assert_eq!(posted.broker_id, "order_id=100500");
        assert_eq!(posted.transactions.len(), 1);

        let t2 = Transaction::new(5, 320.0);
        posted.add_transaction(t2);
        assert_eq!(posted.broker_id, "order_id=100500");
        assert_eq!(posted.transactions.len(), 2);

        let dt = Utc::now();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let order = posted.fill(ts, 3.2);
        assert_eq!(order.operation.dt(), dt);
        assert_eq!(order.operation.quantity, 10);
        assert_eq!(order.operation.value, 3200.0);
        assert_eq!(order.operation.commission, 3.2);
    }
    #[test]
    fn reject() {
        let new = MarketOrder::new(Direction::Sell, 10);
        assert_eq!(new.direction, Direction::Sell);
        assert_eq!(new.lots, 10);
        dbg!(&new);

        let reject = new.reject("market is closed");
        assert_eq!(reject.meta, "market is closed");
        dbg!(&reject);
    }
}
