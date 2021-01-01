use crate::Region;
use core::cmp::Ordering;
use substring::Substring;

#[derive(Clone, Debug)]
struct StrPos {
    line: usize,
    col: usize,
    pos: Option<usize>,
}

impl Ord for StrPos {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.line.cmp(&other.line) {
            Ordering::Equal => self.col.cmp(&other.col),
            ord => ord,
        }
    }
}

impl PartialOrd for StrPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for StrPos {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.col == other.col
    }
}

impl Eq for StrPos {}

impl StrPos {
    fn new(line: usize, col: usize) -> Self {
        Self {
            line,
            col,
            pos: None,
        }
    }

    fn advance(&mut self, next: (usize, char)) {
        self.pos = Some(next.0 + 1);
        if next.1 == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }
}

fn find_all(string: &str, positions: &mut [StrPos]) {
    positions.sort();
    let mut next_pos_index = 0;

    let mut current_pos = StrPos::new(1, 1);
    current_pos.pos = Some(0);
    let mut chars = string.char_indices();

    loop {
        match positions.get_mut(next_pos_index) {
            Some(next_pos) => {
                if current_pos >= *next_pos {
                    next_pos.pos = current_pos.pos;
                    next_pos_index += 1;
                    continue;
                }
            }
            None => break,
        }
        match chars.next() {
            Some(c) => current_pos.advance(c),
            None => panic!(),
        }
    }
}

pub fn get_region_text(regions: Vec<Region>, file_content: &str) -> Vec<(Region, &str)> {
    let mut positions = Vec::<StrPos>::new();

    for region in regions.iter() {
        let start_pos = StrPos::new(region.start.0, region.start.1);
        let end_pos = StrPos::new(region.end.0, region.end.1);

        positions.push(start_pos);
        positions.push(end_pos);
    }

    find_all(file_content, &mut positions);

    let mut output = Vec::new();

    for region in regions.into_iter() {
        let start_pos = StrPos::new(region.start.0, region.start.1);
        let start_pos = positions.binary_search(&start_pos).unwrap();
        let start_pos = positions.get(start_pos).unwrap();
        let start_pos = start_pos.pos.unwrap();

        let end_pos = StrPos::new(region.end.0, region.end.1);
        let end_pos = positions.binary_search(&end_pos).unwrap();
        let end_pos = positions.get(end_pos).unwrap();
        let end_pos = end_pos.pos.unwrap();

        let region_substring = file_content.substring(start_pos, end_pos);
        output.push((region, region_substring));
    }

    output
}

pub fn shrinkwrap(region: Region, region_str: &str) -> Region {
    // all set to start.
    let (mut start_line, mut start_col) = region.start;
    let (mut end_line, mut end_col) = region.start;
    let (mut end_line_running, mut end_col_running) = region.start;

    let mut have_seen_non_whitespace = false;

    for c in region_str.chars() {
        if c.is_whitespace() {
            if !have_seen_non_whitespace {
                if c == '\n' {
                    start_line += 1;
                    start_col = 1;
                } else {
                    start_col += 1;
                }
            }

            if c == '\n' {
                end_line_running += 1;
                end_col_running = 1;
            } else {
                end_col_running += 1;
            }
        } else {
            have_seen_non_whitespace = true;
            end_col_running += 1;

            end_line = end_line_running;
            end_col = end_col_running;
        }
    }

    Region {
        id: region.id,
        start: (start_line, start_col),
        end: (end_line, end_col),
        count: region.count,
        has_count: region.has_count,
        is_gap: region.is_gap,
    }
}
