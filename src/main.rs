use std::io::Read;
use crate::string_seek::shrinkwrap;
use crate::string_seek::get_region_text;
use defaultmap::DefaultBTreeMap;
use std::error::Error;
use std::path::Path;

mod llvm;
mod codecov;
mod string_seek;

#[derive(Clone)]
pub struct Region {
    id: usize,
    start: (usize, usize),
    end: (usize, usize),
    count: u64,
    has_count: bool,
    is_gap: bool,
}

struct OpenRegion {
    id: usize,
    start: (usize, usize),
    count: u64,
    has_count: bool,
    is_gap: bool,
}

impl OpenRegion {
    fn close(&self, end: (usize, usize)) -> Region {
        Region {
            id: self.id,
            start: self.start,
            end,
            count: self.count,
            has_count: self.has_count,
            is_gap: self.is_gap,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut in_buf = String::new();
    std::io::stdin().read_to_string(&mut in_buf)?;
    let in_str = in_buf.as_str();

    let llvm_cov: llvm::LLVMCov = serde_json::from_str(in_str)?;
    let mut codecov = codecov::CodeCov::new();


    
    for file in llvm_cov.data.first().unwrap().files.iter() {
        // These are the llvm regions but cut by sub-regions. They do not overlap.
        let mut region_list = Vec::<Region>::new();

        // These are exactly the llvm regions. They overlap
        let mut region_stack = Vec::<OpenRegion>::new();
        let mut next_region_id = 0;

        // create a sequence of non overlapping regions with coverage info.
        for segment in file.segments.iter() {
            if let Some(r) = handle_segment(&mut region_stack, &segment, &mut next_region_id) {
                region_list.push(r);
            }
        }

        let mut line_coverage = DefaultBTreeMap::<usize, codecov::CodeCovLineCoverage>::default();

        let file_path = Path::new(file.filename);

        match std::fs::read_to_string(file_path) {
            Ok(file_content) => {
                region_list = get_region_text(region_list, &file_content).into_iter().map(|(r, s)| shrinkwrap(r, s)).collect()
            },
            Err(e) => {
                let file_path = file_path.to_string_lossy();
                let current_dir = std::env::current_dir().unwrap().to_string_lossy().to_string();
                panic!("Error reading file \"{}\": {}. current working directory is \"{}\"", file_path , e, current_dir)
            },
        };

        for region in region_list {
            if region.has_count && !region.is_gap {
                let range = region.start.0 .. region.end.0 + 1;
                for line_num in range {
                    let hit = codecov::CodeCovLineHit {
                        start_col: if line_num == region.start.0 { Some(region.start.1) } else { None },
                        end_col: if line_num == region.end.0 { Some(region.end.1) } else { None },
                        count: region.count,
                    };
                    line_coverage.get_mut(line_num).hit(hit);
                }
            }
        }

        codecov.coverage.insert(file.filename, line_coverage.into());
    }

    let writer = std::io::stdout();
    serde_json::to_writer_pretty(writer, &codecov)?;

    Ok(())
}

fn handle_segment(stack: &mut Vec<OpenRegion>, segment: &llvm::LLVMCovSegment, next_region_id: &mut usize) -> Option<Region> {
    let end = (segment.line, segment.col);
    let new_region = stack.last().map(|r| r.close(end));

    if segment.is_region_entry {
        stack.push(OpenRegion {
            id: *next_region_id,
            start: (segment.line, segment.col),
            count: segment.count,
            has_count: segment.has_count,
            is_gap: segment.is_gap_region,
        });
        *next_region_id += 1;
    } else {
        stack.pop().unwrap();
        if let Some(top) = stack.last_mut() {
            top.start = end;
        }
    }

    new_region
}
