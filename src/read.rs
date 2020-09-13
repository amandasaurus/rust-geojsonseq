use crate::Error;

use jsonseq::JsonSeqReader;
use std::io::Read;

/// Reads GeoJSON objects from a `Read`
pub struct GeoJsonSeqReader<R: Read> {
    inner: JsonSeqReader<R>,
}

impl<R: Read> GeoJsonSeqReader<R> {
    /// Create from this reader
    pub fn new(inner: R) -> Self {
        GeoJsonSeqReader {
            inner: JsonSeqReader::new(inner),
        }
    }

    /// Read the next GeoJSON object from this
    pub fn next_item(&mut self) -> Result<Option<geojson::GeoJson>, Error> {
        match self.inner.next_item()? {
            None => Ok(None),
            Some(json_value) => {
                let o = geojson::GeoJson::from_json_value(json_value)?;
                Ok(Some(o))
            }
        }
    }

    /// Read the next GeoJSON object from this
    pub fn read_item(&mut self) -> Result<Option<geojson::GeoJson>, Error> {
        self.next_item()
    }

    /// Reference to the inner `Read`
    pub fn get_ref(&self) -> &R {
        &self.inner.get_ref()
    }

    /// Mutable reference to the inner `Read`
    pub fn get_mut(&mut self) -> &mut R {
        self.inner.get_mut()
    }

    /// Consume & return the inner reader
    pub fn into_inner(self) -> R {
        self.inner.into_inner()
    }
}

impl<R: Read> Iterator for GeoJsonSeqReader<R> {
    type Item = Result<geojson::GeoJson, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().transpose()
    }
}

impl<R: Read> From<R> for GeoJsonSeqReader<R> {
    fn from(reader: R) -> Self {
        GeoJsonSeqReader::new(reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geojson::{Feature, GeoJson, Geometry, Value};
    use jsonseq::serde_json;
    use std::io::Cursor;

    macro_rules! assert_input_output {
        ( $name:ident, $input:expr, $expected_output:expr) => {
            #[test]
            fn $name() {
                let input: Vec<Vec<u8>> = $input;
                let input: Vec<u8> = input.into_iter().flat_map(|b| b.into_iter()).collect();
                let rdr = GeoJsonSeqReader::new(Cursor::new(input));
                let output = rdr.into_iter().collect::<Result<Vec<_>, _>>().unwrap();
                let expected_output: Vec<GeoJson> = $expected_output.into();
                assert_eq!(output, expected_output);
            }
        };
    }

    assert_input_output!(empty1, vec![], vec![]);
    assert_input_output!(empty2, vec![vec![0x1e, 0x1e]], vec![]);

    assert_input_output!(
        basic1,
        vec![vec![0x1e], r#"{"type":"Feature", "properties":{}, "geometry":{"type":"Point", "coordinates":[0, 0]}}"#.bytes().collect(), vec![0x0A]],

        vec![GeoJson::Feature(Feature {
            bbox: None,
            geometry: Some(Geometry::new(Value::Point(vec![0., 0.]))),
            id: None,
            properties: Some(serde_json::Map::new()),
            foreign_members: None, })]
    );

    assert_input_output!(
        basic2,
        vec![
            vec![0x1e], r#"{"type":"Feature", "properties":{}, "geometry":{"type":"Point", "coordinates":[0, 0]}}"#.bytes().collect(), vec![0x0A],
            vec![0x1e], r#"{"type":"Feature", "properties":{}, "geometry":{"type":"Point", "coordinates":[10, 10]}}"#.bytes().collect(), vec![0x0A],
        ],

        vec![
            GeoJson::Feature(Feature {
            bbox: None,
            geometry: Some(Geometry::new(Value::Point(vec![0., 0.]))),
            id: None,
            properties: Some(serde_json::Map::new()),
            foreign_members: None, }),
            GeoJson::Feature(Feature {
            bbox: None,
            geometry: Some(Geometry::new(Value::Point(vec![10., 10.]))),
            id: None,
            properties: Some(serde_json::Map::new()),
            foreign_members: None, }),
        ]
    );

    assert_input_output!(
        basic3,
        vec![vec![0x1e], r#"{"type":"Feature", "properties":{}, "geometry":{"type":"Point", "coordinates":[0, 0]}}"#.bytes().collect()],

        vec![GeoJson::Feature(Feature {
            bbox: None,
            geometry: Some(Geometry::new(Value::Point(vec![0., 0.]))),
            id: None,
            properties: Some(serde_json::Map::new()),
            foreign_members: None, })]
    );

    #[test]
    fn error1() {
        let input: Vec<u8> = vec![0x1e, '{' as u8];
        let rdr = GeoJsonSeqReader::new(Cursor::new(input));
        let output = rdr.into_iter().collect::<Result<Vec<_>, _>>();
        assert!(matches!(output, Err(Error::JsonError { .. })));
    }

    #[test]
    fn error2() {
        let input: Vec<u8> = vec![0x1e, '{' as u8, '}' as u8];
        let rdr = GeoJsonSeqReader::new(Cursor::new(input));
        let output = rdr.into_iter().collect::<Result<Vec<_>, _>>();
        assert!(matches!(output, Err(Error::GeoJsonError { .. })));
    }
}
