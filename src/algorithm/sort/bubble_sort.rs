/**
冒泡排序：
1、比较相邻的元素。如果第一个比第二个大，就交换它们两个；
2、对每一对相邻元素作同样的工作，从开始第一对到结尾的最后一对，这样在最后的元素应该会是最大的数；
3、针对所有的元素重复以上的步骤，除了最后一个；
4、重复步骤1~3，直到排序完成。
**/

/// use Standard Library
use std::ptr;

/// 升序 Ascending
pub fn asc<T: PartialOrd>(arr: &mut [T]) -> &mut [T] {
    sort(arr, true)
}

/// 降序 Descending
pub fn desc<T: PartialOrd>(arr: &mut [T]) -> &mut [T] {
    sort(arr, false)
}

/// PartialOrd 表示 T 的值是可比较的。
/// ptr::swap 交换指针的值。
/// 参考：
/// https://kaisery.github.io/trpl-zh-cn/ch10-01-syntax.html
/// https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html
pub fn sort<T: PartialOrd>(arr: &mut [T], is_asc: bool) -> &mut [T] {
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
                    // 有些情况下所有权不允许转移，如vec中元素
                    // let temp = arr[j];
                    // arr[j] = arr[j + 1];
                    // arr[j + 1] = temp;

                    // arr.swap(j, j + 1);
                    let pa: *mut T = &mut arr[j];
                    let pb: *mut T = &mut arr[j + 1];
                    unsafe {
                        ptr::swap(pa, pb);
                    }

                    flag = false;
                }
            } else {
                // 降序：从大到小
                if arr[j] < arr[j + 1] {
                    // 有些情况下所有权不允许转移，如vec中元素
                    // let temp = arr[j];
                    // arr[j] = arr[j + 1];
                    // arr[j + 1] = temp;

                    // arr.swap(j, j + 1);
                    let pa: *mut T = &mut arr[j];
                    let pb: *mut T = &mut arr[j + 1];
                    unsafe {
                        ptr::swap(pa, pb);
                    }

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

    let mut a = [
        "a".to_string(),
        "c".to_string(),
        "d".to_string(),
        "f".to_string(),
        "e".to_string(),
        "b".to_string(),
    ];
    println!("{:?}", a);
    asc(&mut a);
    println!("{:?}", a);
}
