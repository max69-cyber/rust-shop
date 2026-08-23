# rust-shop

Учебный проект на Rust — простой магазин с CLI-интерфейсом. Пишется как площадка для изучения языка: ownership и borrowing, structs/enums, traits, generics, модули, обработку ошибок, итераторы и тесты.

## Что внутри

- **Каталог товаров** — список для покупки, с ценой и описанием
- **Пользователь с балансом** — покупка списывает деньги, при нехватке средств — отказ
- **Инвентарь** — история покупок с итоговой суммой потраченного
- **CLI-интерфейс** — навигация по экранам через номера пунктов меню

## Скриншоты

<img width="918" height="660" alt="2026-08-23_5-36-26 PM" src="https://github.com/user-attachments/assets/5c0dbab5-07cf-4217-bea9-edf03d150be7" />
<img width="902" height="653" alt="image" src="https://github.com/user-attachments/assets/6f7958a2-47d7-44fe-a900-6298276f82e4" />
<img width="904" height="654" alt="image" src="https://github.com/user-attachments/assets/9334812e-9699-46df-b550-6efa1ed9a389" />


## Как запустить

Нужен установленный [Rust](https://www.rust-lang.org/tools/install) (`rustc`/`cargo`).

```bash
git clone https://github.com/max69-cyber/rust-shop.git
cd rust-shop
cargo run
```

## Тесты

```bash
cargo test
```

## Структура проекта

```
src/
  main.rs                          точка входа
  domain.rs, domain/                бизнес-сущности и правила (Product, Order, User)
  repository.rs, repository/        generic-хранилище (Repository<T>) и его использование
  ui.rs, ui/                        CLI: экраны, ввод, вывод
```
