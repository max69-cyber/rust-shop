pub struct Order {
    id: String,
    price: f32,
    description: String,
}

impl Order {
    pub fn new(id: String, price: f32, description: String) -> Order {
        Order {
            id,
            price,
            description,
        }
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
