fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; // This is the error. Multiple mutable borrows!
    *y = 10;
    *z = 100;
}