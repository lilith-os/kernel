use thiserror_no_std::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Test failed with error: {}", .0)]
    TestError(&'static str),
}