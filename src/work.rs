use crate::connection::Connection;

pub enum Command {
    Write(Connection),
    Read(Connection),
}
