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

// отдельный модуль для тестов, используется только при запуске тестов
#[cfg(test)]
mod tests {
    //можно пользоваться всем публичным текущего модуля
    use super::*;

    // сам тест
    #[test]
    fn creates_valid_product() {
        let product = Product::new(
            "ART-1".to_string(),
            "Macbook".to_string(),
            1000.0,
            "cool".to_string(),
        );

        // проверка результата, в данном случае того, что пришло Ok
        assert!(product.is_ok());
    }

    #[test]
    fn rejects_empty_name() {
        let result = Product::new(
            "ART-1".to_string(),
            "   ".to_string(),
            1000.0,
            "cool".to_string(),
        );

        assert!(matches!(result, Err(ProductError::EmptyNameError)));
    }

    #[test]
    fn rejects_zero_price() {
        let result = Product::new(
            "ART-1".to_string(),
            "Macbook".to_string(),
            0.0,
            "cool".to_string(),
        );

        assert!(matches!(result, Err(ProductError::InvalidPriceError)));
    }

    #[test]
    fn rejects_negative_price() {
        let result = Product::new(
            "ART-1".to_string(),
            "Macbook".to_string(),
            -5.0,
            "cool".to_string(),
        );

        assert!(matches!(result, Err(ProductError::InvalidPriceError)));
    }

    #[test]
    fn getters_return_correct_values() {
        let product = Product::new(
            "ART-1".to_string(),
            "Macbook".to_string(),
            1500.0,
            "cool laptop".to_string(),
        )
        .unwrap();

        assert_eq!(product.get_article(), "ART-1");
        assert_eq!(product.get_name(), "Macbook");
        assert_eq!(product.get_price(), 1500.0);
        assert_eq!(product.get_description(), "cool laptop");
    }
}
