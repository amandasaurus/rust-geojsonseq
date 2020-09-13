use jsonseq::{serde_json, JsonSeqWriter};
use std::io::Write;

use crate::Error;

/// Writes GeoJSON objects from a `Write`
pub struct GeoJsonSeqWriter<R: Write> {
    inner: JsonSeqWriter<R>,
}

impl<W: Write> GeoJsonSeqWriter<W> {
    /// Create from this reader
    pub fn new(inner: W) -> Self {
        GeoJsonSeqWriter {
            inner: JsonSeqWriter::new(inner),
        }
    }

    /// Write a GeoJSON object to this
    pub fn write_object(&mut self, o: geojson::GeoJson) -> Result<(), Error> {
        let json_o: serde_json::Value = serde_json::to_value(o)?;
        self.inner.write_item(&json_o)?;

        Ok(())
    }

    pub fn get_ref(&self) -> &W {
        &self.inner.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut W {
        self.inner.get_mut()
    }

    pub fn into_inner(self) -> W {
        self.inner.into_inner()
    }
}
