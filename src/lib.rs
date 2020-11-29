#![crate_name = "rsutils"] // 库的名称为 rsutils
#![crate_type = "lib"] // 这个 crate 是一个库文件

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod math;
