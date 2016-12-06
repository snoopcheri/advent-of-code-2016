pub struct Triangle {
    length1: u32,
    length2: u32,
    length3: u32,
}


impl Triangle {
    pub fn new(length1: u32, length2: u32, length3: u32) -> Triangle {
        Triangle {
            length1: length1,
            length2: length2,
            length3: length3
        }
    }

    pub fn is_valid(&self) -> bool {
        true
            && self.length1 + self.length2 > self.length3
            && self.length1 + self.length3 > self.length2
            && self.length2 + self.length3 > self.length1
    }
}
