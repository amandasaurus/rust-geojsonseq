use jsonseq::serde_json;
use thiserror::Error;

pub mod read;
pub mod write;

pub use read::GeoJsonSeqReader;
pub use write::GeoJsonSeqWriter;

/// An error with reading or writing
#[derive(Debug, Error)]
pub enum Error {
    /// I/O Error from the underlying `Read`
    #[error("I/O Error from the underlying `Read`")]
    IOError(#[from] std::io::Error),

    /// Data was read OK, but there was a problem parsing as JSON
    #[error("Data was read OK, but there was a problem parsing as JSON")]
    JsonError(#[from] serde_json::Error),

    /// Valid JSON as read, but not valid as a GeoJSON object
    #[error("Valid JSON as read, but not valid as a GeoJSON object")]
    GeoJsonError(#[from] geojson::Error),
}

impl From<jsonseq::Error> for Error {
    fn from(e: jsonseq::Error) -> Error {
        match e {
            jsonseq::Error::IOError(e) => Error::IOError(e),
            jsonseq::Error::JsonError(e) => Error::JsonError(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geojson::*;
    use std::io::Cursor;

    #[test]
    fn roundtrip1() {
        let buf = Vec::new();
        let mut wtr = GeoJsonSeqWriter::new(buf);

        let geojson1 = GeoJson::Feature(Feature {
            bbox: None,
            geometry: Some(Geometry::new(Value::Point(vec![0., 0.]))),
            id: None,
            properties: Some(serde_json::Map::new()),
            foreign_members: None,
        });

        wtr.write_object(geojson1.clone()).unwrap();

        let buf = wtr.into_inner();
        let mut rdr = GeoJsonSeqReader::new(Cursor::new(buf));

        assert_eq!(rdr.read_item().unwrap().unwrap(), geojson1);
        assert_eq!(rdr.read_item().unwrap(), None);
    }
}
