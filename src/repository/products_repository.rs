use crate::{domain::product::Product, repository::RepositoryError};
use std::collections::HashMap;

// хранилище списка продуктов
pub struct ProductsRepository {
    // для более быстрого поиска по товарам используется мапа по артикулам
    products: HashMap<String, Product>,
}

impl ProductsRepository {
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
        }
    }

    /// добавляет в хранилище товар. кидает ошибку в случае если товар с артикулом уже существует
    pub fn add(&mut self, product: Product) -> Result<(), RepositoryError> {
        let key = product.get_article();
        if self.products.contains_key(key) {
            return Err(RepositoryError::AddingDuplicateItemError);
        };

        self.products
            .insert(product.get_article().to_string(), product);

        Ok(())
    }

    /// проверяет, есть ли в репо товар по артикулу
    pub fn exists(&self, article: &str) -> bool {
        self.products.contains_key(article)
    }

    /// находит товар по артикулу и выдает его ссылкой. если его нет - вернет None
    pub fn find(&self, article: &str) -> Option<&Product> {
        self.products.get(article)
    }

    /// Возвращает вектор ссылок на продукт
    pub fn get_list(&self) -> Vec<&Product> {
        // values - итератор по содержанию мапы, collect преобразует его в вектор
        self.products.values().collect()
    }
}
