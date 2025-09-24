use strum_macros::Display;
use thiserror::Error;

#[derive(Error, Debug, Display)]
pub enum Object {
	ParseObjectTypeError,
}
