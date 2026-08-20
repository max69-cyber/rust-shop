pub struct Order {
    name: String,
    price: f32,
    description: String,
}

#[derive(Debug)]
pub enum OrderError {
    EmptyNameError,
    InvalidPriceError,
}

impl Order {
    pub fn new(id: String, price: f32, description: String) -> Result<Order, OrderError> {
        if id.trim().is_empty() {
            // тут придется потрудиться и написать return, ибо это не последнее выражение как Ok
            return Err(OrderError::EmptyNameError);
        }
        if price <= 0f32 {
            return Err(OrderError::InvalidPriceError);
        }

        Ok(Order {
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
