macro_rules! try_ready_receive {
    ($e:expr) => {
        match $e {
            Ok(::futures::Async::Ready(v)) => v,
            Ok(::futures::Async::NotReady) => return Ok(::futures::Async::NotReady),
            Err(()) => unreachable!("mpsc::Receiver doesn't return errors"),
        }
    };
}

macro_rules! try_ready_closed {
    ($e:expr) => {
        match $e {
            Ok(::futures::Async::Ready(v)) => v,
            Ok(::futures::Async::NotReady) => return Ok(::futures::Async::NotReady),
            Err(_) => return Err(crate::Error::closed()),
        }
    };
}

mod bind;
mod cancel;
mod client;
mod codec;
mod connect;
mod connection;
mod copy_in;
mod copy_out;
mod execute;
mod portal;
mod prepare;
mod query;
mod row;
mod simple_query;
mod statement;
mod tls;
mod transaction;
mod typeinfo;
mod typeinfo_composite;
mod typeinfo_enum;

pub use crate::proto::bind::BindFuture;
pub use crate::proto::cancel::CancelFuture;
pub use crate::proto::client::Client;
pub use crate::proto::codec::PostgresCodec;
pub use crate::proto::connect::ConnectFuture;
pub use crate::proto::connection::Connection;
pub use crate::proto::copy_in::CopyInFuture;
pub use crate::proto::copy_out::CopyOutStream;
pub use crate::proto::execute::ExecuteFuture;
pub use crate::proto::portal::Portal;
pub use crate::proto::prepare::PrepareFuture;
pub use crate::proto::query::QueryStream;
pub use crate::proto::row::Row;
pub use crate::proto::simple_query::SimpleQueryFuture;
pub use crate::proto::statement::Statement;
pub use crate::proto::tls::TlsFuture;
pub use crate::proto::transaction::TransactionFuture;
