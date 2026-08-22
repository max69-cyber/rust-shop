pub struct Product {
    article: String,
    name: String,
    price: f32,
    description: String,
}

#[derive(Debug)]
pub enum ProductError {
    EmptyNameError,
    InvalidPriceError,
}

impl Product {
    pub fn new(
        article: String,
        name: String,
        price: f32,
        description: String,
    ) -> Result<Product, ProductError> {
        if name.trim().is_empty() {
            // тут придется потрудиться и написать return, ибо это не последнее выражение как Ok
            return Err(ProductError::EmptyNameError);
        }
        if price <= 0f32 {
            return Err(ProductError::InvalidPriceError);
        }

        Ok(Product {
            article,
            name,
            price,
            description,
        })
    }

    // геттеры приватных полей
    pub fn get_article(&self) -> &str {
        &self.article
    }
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
