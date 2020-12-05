// 升序 Ascending
pub fn asc<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    sort(arr, true)
}

// 降序 Descending
pub fn desc<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    sort(arr, false)
}

pub fn sort<T: PartialOrd + Copy>(arr: &mut [T], is_asc: bool) -> &mut [T] {
    if arr.len() == 0 {
        return arr;
    }

    let mut flag: bool;
    for i in 0..arr.len() - 1 {
        flag = true;

        for j in 0..arr.len() - 1 - i {
            if is_asc {
                // 升序：从小到大
                if arr[j] > arr[j + 1] {
                    let temp = arr[j];
                    arr[j] = arr[j + 1];
                    arr[j + 1] = temp;
                    flag = false;
                }
            } else {
                // 降序：从大到小
                if arr[j] < arr[j + 1] {
                    let temp = arr[j];
                    arr[j] = arr[j + 1];
                    arr[j + 1] = temp;
                    flag = false;
                }
            }
        }

        // 无位置变动时，说明排序已完成，可提前退出
        if flag {
            break;
        }
    }

    return arr;
}

#[test]
fn sort_test() {
    let mut a = [2, 5, 3, 1, 4];
    println!("{:?}", a);
    asc(&mut a);
    println!("{:?}", a);

    let mut a = [0.2, 0.5, 0.3, 0.1, 0.4];
    println!("{:?}", a);
    desc(&mut a);
    println!("{:?}", a);

    let mut a = ['2', '5', '3', '1', '4'];
    println!("{:?}", a);
    asc(&mut a);
    println!("{:?}", a);

    let mut a = ["29", "22", "22", "5", "3", "1", "4"];
    println!("{:?}", a);
    desc(&mut a);
    println!("{:?}", a);

    let mut a = ["a", "c", "d", "f", "e", "b"];
    println!("{:?}", a);
    asc(&mut a);
    println!("{:?}", a);
}
