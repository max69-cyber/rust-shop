pub struct Order {
    id: String,
    price: f32,
    description: String,
}

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
            id,
            price,
            description,
        })
    }

    // геттеры приватных полей
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
