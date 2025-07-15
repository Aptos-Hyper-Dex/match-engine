use pricelevel::OrderId;
use std::cell::RefCell;

/// A memory pool for reusing vectors to reduce allocations in hot paths.
#[derive(Debug)]
pub struct MatchingPool {
    filled_orders_pool: RefCell<Vec<Vec<OrderId>>>,
    price_vec_pool: RefCell<Vec<Vec<u64>>>,
}

impl MatchingPool {
    /// Creates a new, empty matching pool.
    pub fn new() -> Self {
        MatchingPool {
            filled_orders_pool: RefCell::new(Vec::with_capacity(4)),
            price_vec_pool: RefCell::new(Vec::with_capacity(4)),
        }
    }

    /// Retrieves a vector for filled orders from the pool.
    pub fn get_filled_orders_vec(&self) -> Vec<OrderId> {
        self.filled_orders_pool
            .borrow_mut()
            .pop()
            .unwrap_or_else(|| Vec::with_capacity(16))
    }

    /// Returns a filled orders vector to the pool for reuse.
    pub fn return_filled_orders_vec(&self, mut vec: Vec<OrderId>) {
        vec.clear();
        self.filled_orders_pool.borrow_mut().push(vec);
    }

    /// Retrieves a vector for prices from the pool.
    pub fn get_price_vec(&self) -> Vec<u64> {
        self.price_vec_pool
            .borrow_mut()
            .pop()
            .unwrap_or_else(|| Vec::with_capacity(32))
    }

    /// Returns a price vector to the pool for reuse.
    pub fn return_price_vec(&self, mut vec: Vec<u64>) {
        vec.clear();
        self.price_vec_pool.borrow_mut().push(vec);
    }
}

impl Default for MatchingPool {
    fn default() -> Self {
        Self::new()
    }
}
