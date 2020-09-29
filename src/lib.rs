mod bencode;
pub mod err;

pub use bencode::BenCodeAble;

pub enum BenCodeType<T: BenCodeAble> {
    BenCodedString(String),
    BenCodedInt(i64),
    BenCodedList(Vec<T>),
}
