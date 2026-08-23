use std::time::SystemTime;

pub struct OrderLegacy {
    name: String,
    price: f32,
    description: String,
}

pub struct Order {
    product_article: String,
    price_paid: f32,
    purchase_time: SystemTime,
}

#[derive(Debug)]
pub enum OrderLegacyError {
    EmptyNameError,
    InvalidPriceError,
}

#[derive(Debug)]
pub enum OrderError {
    NoSuchProductError,
    InvalidPriceError,
}

impl OrderLegacy {
    pub fn new(
        id: String,
        price: f32,
        description: String,
    ) -> Result<OrderLegacy, OrderLegacyError> {
        if id.trim().is_empty() {
            // тут придется потрудиться и написать return, ибо это не последнее выражение как Ok
            return Err(OrderLegacyError::EmptyNameError);
        }
        if price <= 0f32 {
            return Err(OrderLegacyError::InvalidPriceError);
        }

        Ok(OrderLegacy {
            name: id,
            price,
            description,
        })
    }

    // геттеры приватных полей
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}

impl Order {
    pub fn new(article: String, price: f32) -> Result<Order, OrderError> {
        if price <= 0f32 {
            return Err(OrderError::InvalidPriceError);
        }

        Ok(Order {
            product_article: article,
            price_paid: price,
            purchase_time: SystemTime::now(),
        })
    }

    // геттеры приватных полей
    pub fn get_product_article(&self) -> &str {
        &self.product_article
    }

    pub fn get_price_paid(&self) -> f32 {
        self.price_paid
    }

    pub fn get_purchase_time(&self) -> SystemTime {
        self.purchase_time
    }
}
