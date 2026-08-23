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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_order() {
        let order = Order::new("ART-1".to_string(), 1000.0);

        assert!(order.is_ok());
    }

    #[test]
    fn rejects_zero_price() {
        let order = Order::new("ART-1".to_string(), 0.0);

        assert!(matches!(order, Err(OrderError::InvalidPriceError)));
    }

    #[test]
    fn rejects_negative_price() {
        let order = Order::new("ART-1".to_string(), -1.0);

        // для enum ошибок надо использовать matches
        assert!(matches!(order, Err(OrderError::InvalidPriceError)));
    }

    #[test]
    fn getters_return_correct_values() {
        let order = Order::new("ART-1".to_string(), 1500.0).unwrap();

        assert_eq!(order.get_product_article(), "ART-1");
        assert_eq!(order.get_price_paid(), 1500.0);
    }
}
