mod get;
pub use get::Get;

mod ping;
pub use ping::Ping;

mod set;
pub use set::Set;

// use crate::Connection;

#[derive(Debug)]
pub enum Command {
    Get(Get),
    Ping(Ping),
}

impl Command {
    // pub(crate) fn get_name(&self) -> &str {
    //     match self {
    //         Command::Get(_) => "get",
    //         Command::Ping(_) => "ping",
    //     }
    // }
}
