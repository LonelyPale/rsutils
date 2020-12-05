use rsutils::algorithm::sort::bubble_sort;

fn main() {
    println!("Hello, world!");
    let mut arr = [1, 9, -99, 0, 2, -3, 7, 2, 4, 8];
    bubble_sort::asc(&mut arr);
    println!("{:?}", arr)
}
