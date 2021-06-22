pub fn org_iter() {
    let v1 = vec![10,20,30];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }
}