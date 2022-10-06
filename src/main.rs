use std::io::stdin;

/*
TODO: написать функцию для считывания, доделать эту хрень (a/c * b/d) - (a*b - c)/c*d
Функции для задачи: .abs(), .sin(), .pow()
*/

//Функция main служит для считывания ввода и коммуникации с пользователем (фронтенд)
fn main() {
    loop { //инициализация петли
        println!("Введите значение переменных в порядке a, b, c, d:"); //вывод сообщения с просьбой ввести значение переменных

        let a:f64 = read_var();
        let b:f64 = read_var();
        let c:f64 = read_var();
        let d:f64 = read_var(); //создание переменных

        let y: f64 = calc(a, b, c, d); //переменная Y приравнивается к функции calc от X
        println!("{y}");
        pause(); //использование фунции паузы
        break; //разрыв петли
    }
}

fn calc(a: f64, b: f64, c: f64, d: f64) -> f64 {
(a/b * b/d) - (a*b - c)/c*d //функция calc выполняет (a/c * b/d) - (a*b - c)/c*d
}

fn read_var() -> f64 {
    loop {
        let mut x =String::new();
        stdin()
            .read_line(&mut x)
            .expect("Ошибка чтения ввода"); //считывание ввода и запись его в переменную X (с точки зрения компилятора это одна строка)

        let x: f64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Переменная Х должна быть равна числу");
                continue;
            },
        }; //преобразование ввода со string в float-point integer с перезапуском loop в случае ошибки
        break x;
    }
}

fn pause() { //фукция паузы
    let mut q = String::new();
    stdin().read_line(&mut q).expect("ошибка");
    println!("нажмите Enter чтобы выйти.");
}