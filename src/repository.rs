use std::collections::HashMap;

pub mod products_repository;

#[derive(Debug)]
pub enum RepositoryError {
    AddingDuplicateItemError,
}

// trait - это шаблон для impl, можно сказать что это типа интерфейс
pub trait Identifiable {
    // чтобы каждый репозиторий смог определить как в нем правильно достать id
    fn get_id(&self) -> String;
}

pub struct Repository<T: Identifiable> {
    items: HashMap<String, T>,
}

impl<T: Identifiable> Repository<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// добавляет в хранилище айтем. кидает ошибку в случае если айтем с ключом уже существует
    pub fn add(&mut self, item: T) -> Result<(), RepositoryError> {
        let key = item.get_id();

        if self.items.contains_key(&key) {
            return Err(RepositoryError::AddingDuplicateItemError);
        };

        self.items.insert(key, item);
        Ok(())
    }

    /// проверяет, есть ли в репо айтем по id
    pub fn exists(&self, id: &str) -> bool {
        self.items.contains_key(id)
    }

    /// находит айтем по id и выдает его ссылкой. если его нет - вернет None
    pub fn find(&self, id: &str) -> Option<&T> {
        self.items.get(id)
    }

    /// возвращает вектор ссылок на продукт
    pub fn get_list(&self) -> Vec<&T> {
        // values - итератор по содержанию мапы, collect преобразует его в вектор
        self.items.values().collect()
    }
}
