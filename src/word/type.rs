/// 以下内容，及构造方式，来自百度百科——
/// [日语-词汇分类](https://baike.baidu.com/item/日语/398720?fr=aladdin)   
/// 在日语中，词类称为“品词”(品詞<ひんし>)，按照意义、形态和在句中的作用，  
/// 可以分为十二类   
/// 日语中的单词总体上可以分为两大类：独立词和附属词   
///    
/// 独立词：   
///    
/// 体言——无词尾变化，其中名词、代名词、数词可做主语   
/// 名词（めいし）：表示人或事物的名称，例词：テレビ、电话、部屋。   
/// 代名词（だいめいし）：用来代替人或事物的名称，
/// 例词：わたし、あなた、彼、彼女。   
/// 数词（すうし）：表示数目和数量的单位，例词：一、一つ。   
/// 副词（ふくし）：修饰用言，例词：たくさん、すごい。   
/// 连体词（れんたいし）：修饰体言，例词：この、あの、その。   
/// 接続词（せつぞくし）：起接续作用，例词：でも、しかし。   
/// 感叹词（かんたんし）：表示感叹，呼唤或应答，例词：はい、ええ、いいえ。   
///    
/// 用言——有词尾变化，可单独作谓语   
/// 动词（どうし）：表示动作、存在或状态，例词：书く、食べる、ある、いる。   
/// 形容词（けいようし）：表示性质或状态，例词：高い、低い、暑い、寒い。   
/// 形容动词（けいようどうし）：表示性质或状态，这是日语当中特有的一种品词，
/// 它具有形容词的功能，但又具有和动词一样的词尾变化，所以叫形容动词。
/// 例词：好きだ、上手だ、静かだ。   
///   
/// 附属词   
/// 助词（じょし）：无词尾变化，附加在词后，表示词的语法地位，
/// 与其它词的关系，增加含义。   
/// 助动词（じょどうし）：有词尾变化，用在用言或助动词后，起一定的语法作用。   
///
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum HinShi {
    MeiShi,
    DaiMeiShi,
    SuuShi,
    FukuShi,
    RenTaiShi,
    SetsuZokuShi,
    KanTanShi,
    DouShi,
    KeiYouShi,
    KeiYouDouShi,
    JyoShi,
    JyoDouShi,
    MiChiShi,
}

impl HinShi {
    // 返回一个未知类型
    fn _michi() -> Self {
        HinShi::MiChiShi
    }
}

#[derive(Debug)]
pub struct ParseHinShiError {
    _priv: (),
}

impl fmt::Display for ParseHinShiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // "nyuuryoku chi ni ayamari ga arimasu"
        "入力値に誤りがあります".fmt(f)
    }
}

impl FromStr for HinShi {
    type Err = ParseHinShiError;

    fn from_str(s: &str) -> Result<Self, ParseHinShiError> {
        match Some(s) {
            Some("MeiShi") => Ok(HinShi::MeiShi),
            Some("DaiMeiShi") => Ok(HinShi::DaiMeiShi),
            Some("SuuShi") => Ok(HinShi::SuuShi),
            Some("FukuShi") => Ok(HinShi::FukuShi),
            Some("RenTaiShi") => Ok(HinShi::RenTaiShi),
            Some("SetsuZokuShi") => Ok(HinShi::SetsuZokuShi),
            Some("KanTanShi") => Ok(HinShi::KanTanShi),
            Some("DouShi") => Ok(HinShi::DouShi),
            Some("KeiYouShi") => Ok(HinShi::KeiYouShi),
            Some("KeiYouDouShi") => Ok(HinShi::KeiYouDouShi),
            Some("JyoShi") => Ok(HinShi::JyoShi),
            Some("JyoDouShi") => Ok(HinShi::JyoDouShi),
            Some("MiChiShi") => Ok(HinShi::MiChiShi),
            _ => Err(ParseHinShiError { _priv: () }),
        }
    }
}
