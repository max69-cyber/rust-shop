// импорт из текущей директории: root (crate) -> order (mod order в main) -> use Order здесь
use super::order::{Order, OrderLegacy};
use super::product::Product;

// User - публичный struct, поля при этом приватные
// struct хранит поля
pub struct User {
    // User является владельцем строки своего имени
    name: String,
    balance: f32,
    // Vec == List в C#
    orders_legacy: Vec<OrderLegacy>,
    orders: Vec<Order>,
}

// для того чтобы смочь использовать unwrap в покупке
#[derive(Debug)]
pub enum UserError {
    EmptyNameError,
    InvalidBalanceError,
}
// можно вынести куда то
pub enum BuyError {
    NotEnoughMoneyError,
}

// для реализации методов создадим impl с тем же названием
impl User {
    /// Конструктор User, получает строку имени и число с запятой баланса
    ///
    /// При этом строка определена в памяти, это именна та строка которую передаем,
    /// а число скопировано с оригинала.
    ///
    /// Если строка понадобится далее в источнике, то можно скопировать ее,
    /// и оставить владение строкой источнику.
    /// Тогда в сигнатуре необходимо написать &str - указатель, и скопировать в создании через
    /// .to_string(), тем самым отдавая во владение User новый инстанс в памяти.
    ///
    /// -> Self - отдаем в переменную которая принимает ответ от конструктора владение самим User.
    ///
    /// Как работает без return - у User нет ";" в конце поэтому он возвращается из функции
    /// Если бы был то значение просто пропало, получается шорткат: return a; == a
    ///
    /// Self - это то же самое что и User тут, ну и вообще так работает для любых struct
    pub fn new(name: String, balance: f32) -> Result<Self, UserError> {
        if name.trim().is_empty() {
            return Err(UserError::EmptyNameError);
        }

        if balance <= 0.0 {
            return Err(UserError::InvalidBalanceError);
        }

        Ok(Self {
            name,
            balance,
            orders_legacy: Vec::new(),
            orders: Vec::new(),
        })
    }

    /// Получить баланс по инстансу, хотим получить ссылку на какой-то инстанс юзера,
    /// что в свою очередь нам позволит тут достучаться до его приватного поля баланса,
    /// потому что мы в том же модуле что и само поле,
    /// и вернуть копию его значения (потому что простой тип данных)
    pub fn get_balance(&self) -> f32 {
        self.balance
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_orders_legacy(&self) -> &[OrderLegacy] {
        &self.orders_legacy
    }
    pub fn get_orders(&self) -> &[Order] {
        &self.orders
    }

    // позже можно создать репозитории для ресов
    pub fn buy_legacy(&mut self, order: OrderLegacy) -> Result<(), BuyError> {
        let order_price = order.get_price();

        if self.balance < order_price {
            return Err(BuyError::NotEnoughMoneyError);
        }

        self.balance -= order_price;
        self.orders_legacy.push(order);

        Ok(())
    }

    pub fn buy(&mut self, product: &Product) -> Result<(), BuyError> {
        let price = product.get_price();
        let article: String = product.get_article().to_string();

        if self.balance < price {
            return Err(BuyError::NotEnoughMoneyError);
        }

        self.balance -= price;

        let order =
            Order::new(article, price).expect("Invalid price is already checked by Product");
        self.orders.push(order);

        Ok(())
    }
}
