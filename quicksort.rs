pub fn quicksort<T: Clone + PartialOrd>(x: &Vec<T>) -> Vec<T> {
    let smaller: Vec<T> = x.iter().filter(|i| **i < x[0]).map(|i| i.clone()).collect();
    let equal:   Vec<T> = x.iter().filter(|i| **i == x[0]).map(|i| i.clone()).collect();
    let greater: Vec<T> = x.iter().filter(|i| **i > x[0]).map(|i| i.clone()).collect();
    if x.len() == 0 {
        vec![]
    } else if x.len() == 1 {
        vec![x[0].clone()]
    } else {
        [quicksort(&smaller), quicksort(&equal), quicksort(&greater)].concat()
    }
}

#[test]
fn neg_check() {
    assert_eq!(quicksort(&vec![1, 0, -32, 3, 51]), vec![-32, 0, 1, 3, 51]);
}
#[test]
fn float_check() {
    assert_eq!(quicksort(&vec![1.0, 0.0, -32.0, 3.0, 51.0]), vec![-32.0, 0.0, 1.0, 3.0, 51.0]);
}
#[test]
fn str_check() {
    assert_eq!(quicksort(&vec!["world", "hello"]), vec!["hello","world"]);
}