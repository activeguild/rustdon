struct Person2 {
    name: String,
    age: u32,
}

impl Person2 {
    fn say_name(&self) {
        println!("I am {}.", self.name)
    }

    fn say_age(&self) {
        println!("I am {} year old.", self.age)
    }
}

impl Person2 {
    fn new(name: &str, age: u32) -> Person2 {
        Person2 {
            name: String::from(name),
            age: age,
        }
    }
}

fn main() {
    // String
    let s1: String = String::from("Hello World!");
    let s2: &str = &s1;
    let _s3: String = s2.to_string();

    // タプル
    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";

    // 配列
    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [7; 3];
    a[1] = b[1];
    a[2] = b[2];
    a = b;
    println!("{:?}", &a[0]);
    println!("{:?}", &a[1]);
    println!("{:?}", &a[2]);
    println!("{:?}", &a[0..3]);

    // ユーザー定義型
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let p = Person {
        name: String::from("Johm"),
        age: 8,
    };
    println!("{:?}", p);

    #[derive(Debug)]
    enum Event {
        Quit,
        // KeyDown(u8),
        MouseDown { x: i32, y: i32 },
    }

    let e1 = Event::Quit;
    let e2 = Event::MouseDown { x: 10, y: 10 };

    println!("{:?}", e1);
    println!("{:?}", e2);

    // 頻出する標準ライブラリの型
    // Option
    #[derive(Debug)]
    pub enum Option<T> {
        None,
        Some(T),
    }

    // #[derive(Debug)]
    // pub enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let result: Result<i32, String> = Ok(200);

    // match result {
    //     Ok(code) => println!("code: {}", code),
    //     Err(err) => println!("Err: {}", err),
    // }

    // if let Ok(code) = result {
    //     println!("code: {}", code)
    // }

    println!("code: {}", result.unwrap_or(-1));

    // Vec
    let v1 = vec![1, 2, 3, 4, 5];
    let v2: Vec<i32> = vec![0, 5];

    println!("{:?}", v1[1]);
    println!("{:?}", v2[1]);

    // Box
    // u8型の配列
    // let byte_array = [b'j', b'e', b'1', b'1', b'o'];
    // printbox(Box::new(byte_array));
    // let byte_array = [b'j', b'e'];
    // printbox(Box::new(byte_array));

    // 変数宣言
    // let mut
    let immut_val = 10;
    let mut mut_val = 20;

    mut_val += immut_val;
    println!("{:?}", mut_val);

    // 制御構文
    let number = 1;
    if 0 < number {
        println!("0 < number");
    } else if number < 0 {
        println!("number < 0");
    } else {
        println!("0 == number");
    }

    // ループ
    let mut count = 0;

    let resultLoop = loop {
        println!("count {}", count);

        count += 1;

        if count == 10 {
            break count;
        }
    };

    let mut countWhile = 0;

    while countWhile < 10 {
        println!("count {}", countWhile);
        countWhile += 1;
    }

    let mut count: i32;

    for count in 0..10 {
        println!("count: {}", count);
    }

    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for element in &array {
        println!("count: {}", element);
    }

    // ループ：ラベル
    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");

            break 'main;
            println!("emd loop start"); // 表示されない
        }
        println!("end loop start"); // 表示されない
    }

    // match
    let i: i32 = 1;
    match i {
        1 => println!("1"),
        _ => println!("misc"),
    }

    // match:enum
    enum Color {
        Red,
        Blue,
        Green,
    }

    const matchedColor: Color = Color::Blue;

    match matchedColor {
        Color::Blue => println!("Blue"),
        _ => println!("misc"),
    }

    // match:result
    let result: Result<i32, String> = Ok(100);

    let resule_number = match result {
        Ok(number) => number,
        Err(message) => {
            println!("Error {}", message);
            -1
        }
    };

    // Range
    for number in 1..5 {
        println!("{}", number);
    }

    // Iterator
    let it = Iter { current: 0, max: 0 };
    for num in it {
        println!("{}", num)
    }

    // fn
    let x = add(1, 2);
    println!("{}", x);

    // impl
    let p = Person2 {
        name: String::from("Taro"),
        age: 20,
    };
    p.say_name();
    p.say_age();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize;

    // next()
    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}

fn printbox(s: Box<u8>) {
    print!("{:?}", s);
}
