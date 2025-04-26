use crate::level::Level;

#[derive(Debug)]
pub struct Player<'a> {
    pub(crate) name:&'a str,
    pub(crate) level: Level,
}
