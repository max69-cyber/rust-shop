// подключаем модуль user по имени файла (так просто потому что лежит в той же папке)
mod order;
mod user;

// Не хочу доставать каждый раз struct User из модуля user - использую use, чтобы сократить до User
use order::Order;
use user::User;

// Очевидно, что это точка входа 😎
fn main() {
    //println!("Hello, world!");
    // Для первого коммита просто создадим пользователя разными способами

    // это строка со 'static lifetime, как можно видеть у нее автоматом вывелся тип &str
    // по причине того что она живет вообще все время пока программа выполняется
    // именно сама память, а не переменная, сама переменная уничтожится после "}".
    //
    // поэтому я не смогу просто так передать ее в User, который хочет владеть строкой имени (String)
    let static_str = "Static String";

    // потому что тут произойдет несоответствие
    // let broken_user_constructor = User::new(static_str, 100);

    // значит нам нужно то, что сможет предаться во власть User - копия 'static (как новый String):
    let static_str_copy = static_str.to_string();

    // первый удачный User - с копией статической строки
    let first_user = User::new(static_str_copy, 100.1);

    // теперь другая строка - она живет до конца своего скоупа (следующая "}")
    // будем отдавать оригинальную строку во владение User в его поле имени
    // крафтится из статической строки, которую так же копируем в String
    let users_string = String::from("User's String");

    // тогда напрямую передаем в конструктор переменную с оригиналом строки
    let user_with_original_name = User::new(users_string, 200.02);
    // все, теперь то что изначально было в users_string принадлежит полю name в User, и эта строка
    // жива пока жив этот самый User, и не зависит от переменной users_string

    // теперь этот вызов дает ошибку, потому что строка уже не в переменной, а в поле name
    //println!("{users_string}")

    // но в принципе, если надо можно клонировать строку и отдать ее в User, и свободно пользоваться
    // исходной строкой дальше:
    let not_users_string = String::from("Not User's String");

    let user_with_cloned_string = User::new(not_users_string.clone(), 300f32);
    // и теперь логируем:
    println!("{not_users_string}");

    // и вот еще вывод баланса, тут синтаксический сахар, инстанс прокидывается в функцию
    println!("Balance: {}", user_with_original_name.get_balance());
    // или же так тоже норм
    let balance = User::get_balance(&user_with_cloned_string);
    println!("Balance: {balance}");

    // Добавил заказы:
    let example_order: Order = Order::new(
        String::from("Macbook M5 Air"),
        1000f32,
        String::from("cool laptop"),
    );

    println!("\n----------------------------------------------------------------------\n");

    println!(
        "id: {}, price: {}, description: {}",
        example_order.get_id(),
        Order::get_price(&example_order),
        example_order.get_description(),
    );
}
