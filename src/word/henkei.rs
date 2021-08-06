/// 变形，日语中使用场合不同时需对单词进行不同的形变
///
#[derive(Clone, Debug)]
pub enum HenKei {
    GenKei,
    Masu,
    Te,
    Ta,
    Nai,
    Michi,
}

impl HenKei {
    fn _michi() -> Self {
        HenKei::Michi
    }
}
