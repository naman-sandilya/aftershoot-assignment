#[derive(Debug)]
pub enum Error {
    FileIOError(std::io::Error),
    ImageError(image::ImageError),
    ExifError(exif::Error),
    Base64DecodeError(base64::DecodeError),
    SQLiteError(rusqlite::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileIOError(e) => {
                write!(f, "Error occured opening File! \n{}", e)
            }
            Error::ImageError(e) => {
                write!(f, "Error occured in opening Image File! \n{}", e)
            }
            Error::ExifError(e) => {
                write!(
                    f,
                    "Error occured in parsing Image File for metadata! \n{}",
                    e
                )
            }
            Error::Base64DecodeError(e) => {
                write!(f, "Error occured in decoding Image File! \n{}", e)
            }
            Error::SQLiteError(e) => {
                write!(f, "Error occured in db! \n{}", e)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::FileIOError(err)
    }
}
impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        Error::ImageError(err)
    }
}

impl From<exif::Error> for Error {
    fn from(err: exif::Error) -> Self {
        Error::ExifError(err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::Base64DecodeError(err)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::SQLiteError(err)
    }
}
