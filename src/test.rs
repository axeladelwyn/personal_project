fn main()
{
    let s = "hello";
    let rev_char_iter = s.chars().rev();
    for c in rev_char_iter
    {
        println!("{}", c);
    }
    // check if vector is empty 
    let vec: Vec<i32> = vec![1,3,4,5,5];
    println!("{}", vec.is_empty());
    // convert integer to string
    let string = 123.to_string();
    println!("{}", string);
    // clone for deep copy
    let comparestring1 = String::from("hello");
    let comparestring2 = comparestring1.clone();
    println!("{}", comparestring2);
    // the length of something...
    let text = String::from("Hello");
    println!("{}", text.len());
    // convert String or &str into an iterator so you can use it for loops!
    let iterator = String::from("abc");
    for c in iterator.chars().rev()
    {
        println!("{}", c);
    }

    // return nth item for an iterator (basically indexing)
    println!("{:?}", iterator.chars().nth(1)); // This will return some('b')
    println!("{:?}", iterator.chars().nth(3)); // return None (Out of bonds)

    // extract the value inside Option or reuslt It it's some or OK
    // If it's None it will panics
    let opt = Some(5);
    println!("{}", opt.unwrap());
    println!("{}", iterator.chars().nth(1).unwrap()); // will return 'b' instead of Some('b')

    // 
    let mut chara = String::from("this is babuga");
    chara.truncate(2);
    println!("{}", chara); // this will print th

}