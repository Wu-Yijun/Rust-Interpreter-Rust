fn main() {
    const III: i32 = 123;
    // static mut JJJ: i32 = ii + 1;
    static mut JJJ: i32 = 456;
    let mut kkk = 789;
    fn test() -> i32 {
        println!("Hello, world!{III},{}", unsafe { JJJ });
        unsafe { JJJ += 1 };
        12
    }
    let mut test2 = || {
        println!("Hello, world!{III},{},{kkk}", unsafe { JJJ });
        unsafe { JJJ += 1 };
        kkk += 1;
        12
    };
    let mut a;
    let s = test();
    a = s;
    let s2 = test();
    println!("s = {}", s);
    println!("s = {}", s2);
    println!("III = {}", III);
    println!("JJJ = {}", unsafe { JJJ });

    test2();
    test2();
    let s = test();
    test2();
    // println!("Hello, world!");
    println!("kkk= {kkk}");
}
