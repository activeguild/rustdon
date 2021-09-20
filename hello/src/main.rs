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
}
