#[derive(Debug)]
pub struct ResponseData<T> {
    pub duration: u128,
    pub data: Option<T>,
}
