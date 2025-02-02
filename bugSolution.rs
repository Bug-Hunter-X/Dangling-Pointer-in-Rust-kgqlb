fn main() {
    let mut v = vec![1, 2, 3];
    { //Adding a scope
        let ptr = v.as_mut_ptr();
        unsafe {
            *ptr = 10;
        }
        println!("First element inside the scope: {}", v[0]);
    }
    //println!("First element outside the scope: {}", v[0]); // This line will cause a compilation error
}