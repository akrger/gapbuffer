pub struct GapBuffer {
    pub size: usize,
    pub start: usize,
    pub end: usize,
    pub buffer: Vec<char>,
    pub point: usize
}

impl GapBuffer {
    pub fn new(size: usize) -> GapBuffer {
        GapBuffer {size: size,
                   start: 0,
                   end: size,
                   buffer: vec!['\0'; size],
                   point: 0}
    }
    
    pub fn size(&self) -> usize {
        self.end - self.start
    }
    pub fn insert(&mut self, ch: char) {
        if self.end - self.start == 0 {
             self.extend_buffer();
        }
        self.place_gap();
        self.buffer[self.start] = ch;
        self.start += 1; 
    }

    fn gap_size(&self) -> usize {
        self.end - self.start
    }
    
    fn char_count(&self) -> usize {
        self.buffer.len() - self.gap_size()+1
    }
    
    pub fn set_point(&mut self, point: usize) {
        assert!(point <= self.char_count());
        self.point = point
    }
    
    pub fn place_gap(&mut self) {
        if self.point == self.start {
            return
        }
        let gap_size = self.gap_size();
        let backup = self.buffer.clone();
        if self.point < self.start {
            let delta = self.start - self.point;
            for i in self.point..self.start   {
                self.buffer[i + gap_size] = backup[i]; 
            }
            self.start -= delta;
            self.end -= delta;
            for i in self.start..self.end {
                self.buffer[i] = '\0'
            }
        }
        else if self.point > self.start {
            let delta = self.point - self.start;
            for i in  self.start..self.point {
                self.buffer[i] = backup[i + gap_size]
            }
            self.start += delta;
            self.end += delta;
            for i in self.start..self.end {
                self.buffer[i] = '\0'
            }
        }
    }

    fn extend_buffer(&mut self) {
        let len = self.buffer.len();
        self.buffer.resize(len * 2, '\0');
        self.start = len;
        self.end = self.buffer.len();
        self.place_gap()
    }
    
    pub fn delete(self) {
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_insert_after_cursor() {
        use GapBuffer;
        let mut buffer = GapBuffer::new(4);
        buffer.insert('a');
        buffer.set_point(1);
        buffer.insert('b'); 
        buffer.insert('u'); 
        assert_eq!(buffer.buffer, vec!['a', 'u', '\u{0}', 'b'])
    }

     #[test]
    fn test_insert_before_cursor() {
        use GapBuffer;
        let mut buffer = GapBuffer::new(4);
        buffer.insert('b');
        buffer.set_point(1);
        buffer.insert('a');
        buffer.set_point(0);
        buffer.insert('c');
        assert_eq!(buffer.buffer, vec!['c', '\u{0}', 'b', 'a'])
    }
    
    #[test]
    fn test_insert_after_gap() {
        use GapBuffer;
        let mut buffer = GapBuffer::new(4);
        buffer.insert('b');
        buffer.set_point(1);
        buffer.insert('a');
        buffer.set_point(0);
        buffer.insert('c');
        //                            0 1             2  3 4
        //                            | |           | |  | |
        assert_eq!(buffer.buffer, vec!['c', '\u{0}', 'b', 'a']);
        //                            0 1  2     3  4 5  6 7
        buffer.set_point(3);
        // 2 - 1 + 4 = 5
        //  (GapEnd - GapStart) + the location
        buffer.insert('d');
        assert_eq!(buffer.buffer, vec!['c', 'b', 'a', 'd'])
    }

    #[test]
    fn test_insert_after_capacity_reached() {
        use GapBuffer;
        let mut buffer = GapBuffer::new(1);
        buffer.insert('a');
        buffer.set_point(1);
        buffer.insert('b');
        assert_eq!(buffer.buffer, vec!['a', 'b']);
        buffer.set_point(2);
        buffer.insert('c');
        assert_eq!(buffer.buffer, vec!['a', 'b', 'c', '\u{0}']);
        buffer.set_point(3);
        buffer.insert('d');
        assert_eq!(buffer.buffer, vec!['a', 'b', 'c', 'd']);
        buffer.set_point(4);
        buffer.insert('e');
        assert_eq!(buffer.buffer, vec!['a', 'b', 'c', 'd', 'e', '\u{0}', '\u{0}', '\u{0}']);
    }
    #[test]
    fn test_insert_before_capacity_reached() {
        use GapBuffer;
        let mut buffer = GapBuffer::new(5);
        buffer.insert('T');
        buffer.set_point(1);
        buffer.insert('e');
        buffer.set_point(2);
        buffer.insert('s');
        buffer.set_point(3);
        buffer.insert('t');
        buffer.set_point(0);
        //                            0 1 2 3  4 5  6 7
        assert_eq!(buffer.buffer, vec!['T', 'e', 's', 't', '\u{0}']);
        buffer.insert(' ');
        assert_eq!(buffer.buffer, vec![' ', 'T', 'e', 's', 't']);
    }
}
