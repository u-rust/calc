use std::io;

fn main() {
    println!("Добро пожаловать в калькулятор!");

    loop {
        println!("Выберите операцию, которую нужно выполнить:");
        println!("1. Сложение (+)");
        println!("2. Вычитание (-)");
        println!("3. Умножение (*)");
        println!("4. Деление (/)");
        println!("5. Выход");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Ошибка чтения строки");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 5 {
            println!("До свидания!");
            break;
        }

        println!("Введите первое число:");

        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Ошибка чтения строки");

        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Введите второе число:");

        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Ошибка чтения строки");

        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 {
            let result = num1 + num2;
            println!("{} + {} = {}", num1, num2, result);
        } else if choice == 2 {
            let result = num1 - num2;
            println!("{} - {} = {}", num1, num2, result);
        } else if choice == 3 {
            let result = num1 * num2;
            println!("{} * {} = {}", num1, num2, result);
        } else if choice == 4 {
            let result = num1 / num2;
            println!("{} / {} = {}", num1, num2, result);
        } else {
            println!("Некорректный выбор операции!");
        }
    }
}