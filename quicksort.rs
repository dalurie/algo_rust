fn main() {
    let x:Vec<i32> = vec![1, 0, -32, 3, 51];
    let y:Vec<f64> = vec![1.0, 0.0, -32.0, 3.0, 51.0];
    let z:Vec<&str> = vec!["world", "hello"];
    println!("quicksort({:?}): = {:?}", x, quicksort(&x));
    println!("quicksort({:?}): = {:?}", y, quicksort(&y));
    println!("quicksort({:?}): = {:?}", z, quicksort(&z));
}

fn quicksort<T: Clone + PartialOrd>(x: &Vec<T>) -> Vec<T> {
    let smaller: Vec<T> = x.iter().filter(|i| i < &&x[0]).map(|i| i.clone()).collect();
    let equal:   Vec<T> = x.iter().filter(|i| i == &&x[0]).map(|i| i.clone()).collect();
    let greater: Vec<T> = x.iter().filter(|i| i > &&x[0]).map(|i| i.clone()).collect();
    if x.len() == 0 {
        vec![]
    } else if x.len() == 1 {
        vec![x[0].clone()]
    } else {
        [quicksort(&smaller), quicksort(&equal), quicksort(&greater)].concat()
    }
}