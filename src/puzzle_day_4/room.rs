use std::collections::HashMap;


pub struct Room {
    encrypted_name: String,
    sector_id: u64,
    checksum: String,
}


impl Room {
    pub fn new(encrypted_name: &str, sector_id: u64, checksum: &str) -> Room {
        Room {
            encrypted_name: removed_dashes(encrypted_name),
            sector_id: sector_id,
            checksum: String::from(checksum),
        }
    }

    pub fn is_checksum_valid(&self) -> bool {
        let mut occurrence: HashMap<char, u32> = HashMap::new();
        for ch in self.encrypted_name.chars() {
            *occurrence.entry(ch).or_insert(0) += 1;
        }

        let mut char_by_occurrence: Vec<(&char, &u32)> = occurrence.iter().collect();
        char_by_occurrence.sort_by(|a, b| if a.1 != b.1 { b.1.cmp(a.1) } else { a.0.cmp(b.0) });

        let calculated_checksum: String = char_by_occurrence.iter()
            .take(5)
            .map(|&(&ch, _)| ch)
            .collect();

        self.checksum == calculated_checksum
    }

    pub fn sector_id(&self) -> u64 {
        self.sector_id
    }
}


fn removed_dashes(string: &str) -> String {
    string.chars()
        .filter(|c| *c != '-')
        .collect()
}
