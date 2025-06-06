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
pub enum LimitOrder {
    New(NewLimitOrder),
    Posted(PostedLimitOrder),
    Filled(FilledLimitOrder),
    Rejected(RejectedLimitOrder),
    Canceled(CanceledLimitOrder),
}
impl LimitOrder {
    pub fn new(direction: Direction, lots: u32, price: f64) -> NewLimitOrder {
        NewLimitOrder {
            direction,
            lots,
            price,
        }
    }
    pub fn from_csv() {}
    pub fn to_csv(&self) -> String {
        let mut csv = "limit;".to_string();
        match self {
            LimitOrder::New(o) => {
                csv.push_str("new;");
                csv.push_str(o.direction.to_str());
                csv.push_str(&o.lots.to_string());
            }
            LimitOrder::Posted(_) => csv.push_str("posted;"),
            LimitOrder::Filled(_) => csv.push_str("filled;"),
            LimitOrder::Rejected(_) => csv.push_str("rejected;"),
            LimitOrder::Canceled(_) => csv.push_str("rejected;"),
        };
        todo!();
    }
}
impl std::fmt::Display for LimitOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::New(order) => write!(f, "{order}"),
            Self::Posted(order) => write!(f, "{order}"),
            Self::Filled(order) => write!(f, "{order}"),
            Self::Canceled(order) => write!(f, "{order}"),
            Self::Rejected(order) => write!(f, "{order}"),
        }
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct NewLimitOrder {
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
}
impl NewLimitOrder {
    pub fn post(self, broker_id: &str) -> PostedLimitOrder {
        PostedLimitOrder {
            direction: self.direction,
            lots: self.lots,
            price: self.price,
            broker_id: broker_id.to_string(),
            transactions: Vec::new(),
        }
    }
    pub fn reject(self, meta: &str) -> RejectedLimitOrder {
        RejectedLimitOrder {
            direction: self.direction,
            lots: self.lots,
            price: self.price,
            meta: meta.to_string(),
        }
    }
}
impl std::fmt::Display for NewLimitOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LimitOrder::New={} {}x{}",
            self.direction, self.lots, self.price
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct PostedLimitOrder {
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
    pub broker_id: String,
    pub transactions: Vec<Transaction>,
}
impl PostedLimitOrder {
    pub fn add_transaction(&mut self, t: Transaction) {
        self.transactions.push(t);
    }
    pub fn fill(self, ts_nanos: i64, commission: f64) -> FilledLimitOrder {
        let operation =
            Operation::from(ts_nanos, &self.transactions, commission);
        FilledLimitOrder {
            direction: self.direction,
            lots: self.lots,
            price: self.price,
            broker_id: self.broker_id,
            transactions: self.transactions,
            operation,
        }
    }
    pub fn cancel(self) -> CanceledLimitOrder {
        CanceledLimitOrder {
            direction: self.direction,
            lots: self.lots,
            price: self.price,
            broker_id: self.broker_id,
            transactions: self.transactions,
        }
    }
}
impl std::fmt::Display for PostedLimitOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LimitOrder::Posted={} {}x{} id={} t={:?}",
            self.direction,
            self.lots,
            self.price,
            self.broker_id,
            self.transactions
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct FilledLimitOrder {
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
    pub broker_id: String,
    pub transactions: Vec<Transaction>,
    pub operation: Operation,
}
impl std::fmt::Display for FilledLimitOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LimitOrder::Filled={} {}x{} id={} t={:?} {}",
            self.direction,
            self.lots,
            self.price,
            self.broker_id,
            self.transactions,
            self.operation
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct CanceledLimitOrder {
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
    pub broker_id: String,
    pub transactions: Vec<Transaction>,
}
impl std::fmt::Display for CanceledLimitOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LimitOrder::Canceled={} {}x{} id={} transactions={:?}",
            self.direction,
            self.lots,
            self.price,
            self.broker_id,
            self.transactions
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct RejectedLimitOrder {
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
    pub meta: String,
}
impl std::fmt::Display for RejectedLimitOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LimitOrder::Rejected={} {}x{} meta={}",
            self.direction, self.lots, self.price, self.meta
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn new_post_fill() {
        let new = LimitOrder::new(Direction::Buy, 2, 4500.0);

        let mut posted = new.post("order_id=100500");
        assert_eq!(posted.broker_id, "order_id=100500");

        let t1 = Transaction::new(1, 4500.0);
        posted.add_transaction(t1);
        assert_eq!(posted.broker_id, "order_id=100500");
        assert_eq!(posted.transactions.len(), 1);

        let t2 = Transaction::new(1, 4510.0);
        posted.add_transaction(t2);
        assert_eq!(posted.broker_id, "order_id=100500");
        assert_eq!(posted.transactions.len(), 2);

        let dt = Utc::now();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let order = posted.fill(ts, 4.5);
        assert_eq!(order.operation.dt(), dt);
        assert_eq!(order.operation.ts_nanos, ts);
        assert_eq!(order.operation.quantity, 2);
        assert_eq!(order.operation.value, 9010.0);
        assert_eq!(order.operation.commission, 4.5);
    }
    #[test]
    fn reject() {
        let new = LimitOrder::new(Direction::Buy, 100, 400.0);
        assert_eq!(new.direction, Direction::Buy);
        assert_eq!(new.lots, 100);
        assert_eq!(new.price, 400.0);
        dbg!(&new);

        let reject = new.reject("not enought money");
        assert_eq!(reject.meta, "not enought money");
        dbg!(&reject);
    }
}
