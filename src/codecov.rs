use std::collections::BTreeMap;
use std::collections::HashMap;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct CodeCov<'a> {
    pub coverage: HashMap<&'a str, BTreeMap<usize, CodeCovLineCoverage>>,
}

#[derive(Clone)]
pub struct CodeCovLineCoverage(Vec<CodeCovLineHit>);

#[derive(Clone)]
pub struct CodeCovLineHit {
    pub start_col: Option<usize>,
    pub end_col: Option<usize>,
    pub count: u64,
}

impl Default for CodeCovLineCoverage {
    fn default() -> Self { Self(Vec::default()) }
}

impl CodeCovLineCoverage {
    pub fn hit(&mut self, hit: CodeCovLineHit) {
        self.0.push(hit);
    }
}

impl<'a> CodeCov<'a> {
    pub fn new() -> Self {
        Self {
            coverage: HashMap::new()
        }
    }
}

impl Serialize for CodeCovLineCoverage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hits = &self.0;

        if hits.len() == 0 {
            serializer.serialize_none()
        } else {
            let regions_hit = hits.iter().filter(|hit| hit.count > 0).count();
            let regions = hits.len();

            if regions_hit == 0 || regions_hit == regions {
                let hits = hits.iter().map(|hit| hit.count).max().unwrap();
                serializer.serialize_u64(hits)
            } else {
                serializer.serialize_str(&format!("{}/{}", regions_hit, regions))
            }
        }
    }
}