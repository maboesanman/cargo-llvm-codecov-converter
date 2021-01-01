use serde::Deserialize;

#[derive(Deserialize)]
pub struct LLVMCov<'a> {
    pub data: Vec<LLVMCovDatum<'a>>,

    #[serde(rename = "type")]
    pub format_type: &'a str,
    pub version: &'a str,
}

#[derive(Deserialize)]
pub struct LLVMCovDatum<'a> {
    #[serde(borrow)]
    pub files: Vec<LLVMCovFile<'a>>,
}

#[derive(Deserialize)]
pub struct LLVMCovFile<'a> {
    pub expansions: Vec<&'a str>,
    pub filename: &'a str,
    pub segments: Vec<LLVMCovSegment>,
}

#[derive(Deserialize)]
#[serde(from = "RawSegment")]
pub struct LLVMCovSegment {
    pub line: usize,
    pub col: usize,
    pub count: u64,
    pub has_count: bool,
    pub is_region_entry: bool,
    pub is_gap_region: bool,
}

#[derive(Deserialize)]
struct RawSegment(usize, usize, u64, bool, bool, bool);

impl std::convert::From<RawSegment> for LLVMCovSegment {
    fn from(raw: RawSegment) -> Self {
        let RawSegment(line, col, count, has_count, is_region_entry, is_gap_region) = raw;

        Self {
            line,
            col,
            count,
            has_count,
            is_region_entry,
            is_gap_region,
        }
    }
}
