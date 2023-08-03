fn main() {
    let src = "2 3 4 * +".to_string();
    let ans = rpn_calc0000123::eval(src).unwrap();
    println!("{}", ans);
}
