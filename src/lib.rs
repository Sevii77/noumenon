use std::ops::{Deref, DerefMut};

pub mod formats;

// ----------

trait NullReader {
	fn null_terminated(&self) -> Result<String, std::str::Utf8Error>;
}

impl NullReader for [u8] {
	fn null_terminated(&self) -> Result<String, std::str::Utf8Error> {
		let p = std::str::from_utf8(&self)?;
		Ok(if let Some(l) = p.find('\0') {&p[0..l]} else {p}.to_owned())
	}
}

trait NullWriter {
	fn null_terminated(&self, len: usize) -> Result<Vec<u8>, SizeError>;
}

impl NullWriter for String {
	fn null_terminated(&self, len: usize) -> Result<Vec<u8>, SizeError> {
		let mut vec = vec![0; len];
		let bytes = self.as_bytes();
		if bytes.len() > len {return Err(SizeError{len: bytes.len() as u32, max_len: len as u32})}
		bytes.into_iter().enumerate().for_each(|(i, v)| vec[i] = *v);
		Ok(vec)
	}
}

#[derive(Copy, Eq, PartialEq, Clone, Debug)]
pub struct SizeError {
	len: u32,
	max_len: u32,
}

impl std::fmt::Display for SizeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "string is too long, {} bytes while max is {}", self.len, self.max_len)
	}
}

impl std::error::Error for SizeError {
	fn description(&self) -> &str {
		"string is too long"
	}
}

// ----------

// pub type Error = Box<dyn std::error::Error>;
#[derive(Debug)]
pub enum Error {
	Str(&'static str),
	Io(std::io::Error),
	Binrw(binrw::Error),
	Image(image::ImageError),
	Size(SizeError),
	Utf8(std::str::Utf8Error),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Str(err) => f.write_str(err),
			Self::Io(err) => write!(f, "{:?}", err),
			Self::Binrw(err) => write!(f, "{:?}", err),
			Self::Image(err) => write!(f, "{:?}", err),
			Self::Size(err) => write!(f, "{:?}", err),
			Self::Utf8(err) => write!(f, "{:?}", err),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::Str(_err) => None,
			Self::Io(err) => err.source(),
			Self::Binrw(err) => err.source(),
			Self::Image(err) => err.source(),
			Self::Size(err) => err.source(),
			Self::Utf8(err) => err.source(),
		}
	}
}

impl From<&'static str> for Error {
	fn from(err: &'static str) -> Self {
		Self::Str(err)
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::Io(err)
	}
}

impl From<binrw::Error> for Error {
	fn from(err: binrw::Error) -> Self {
		Self::Binrw(err)
	}
}

impl From<image::ImageError> for Error {
	fn from(err: image::ImageError) -> Self {
		Self::Image(err)
	}
}

impl From<SizeError> for Error {
	fn from(err: SizeError) -> Self {
		Self::Size(err)
	}
}

impl From<std::str::Utf8Error> for Error {
	fn from(err: std::str::Utf8Error) -> Self {
		Self::Utf8(err)
	}
}

// ----------

// TODO: own game data reader, drop iromworks as it is barely used
pub struct Noumenon(ironworks::Ironworks);

impl Deref for Noumenon {
	type Target = ironworks::Ironworks;
	
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Noumenon {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

pub fn get_noumenon() -> Noumenon {
	Noumenon(ironworks::Ironworks::new()
		// .with_resource(ironworks::sqpack::SqPack::new(ironworks::ffxiv::FsResource::at(std::env::current_exe().unwrap().parent().unwrap().parent().unwrap()))))
		.with_resource(ironworks::sqpack::SqPack::new(ironworks::ffxiv::FsResource::at(std::path::Path::new("D:/SteamLibrary/steamapps/common/FINAL FANTASY XIV Online")))))
}