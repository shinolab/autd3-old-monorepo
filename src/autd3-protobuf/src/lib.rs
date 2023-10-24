mod pb {
    tonic::include_proto!("autd3");
}

mod error;
mod traits;

pub use error::*;
pub use traits::*;

pub use pb::*;
