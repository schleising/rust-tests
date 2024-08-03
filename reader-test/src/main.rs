use std::io::Read;

struct MyReader {
    data: Vec<u8>,
    pos: usize,
}

impl MyReader {
    fn new(data: Vec<u8>) -> MyReader {
        MyReader { data, pos: 0 }
    }
}

impl Read for MyReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = std::cmp::min(buf.len(), self.data.len() - self.pos);
        buf[..len].copy_from_slice(&self.data[self.pos..self.pos + len]);
        self.pos += len;
        println!(
            "Reader Length: {}, Reader Pos: {}",
            self.data.len(),
            self.pos
        );
        Ok(len)
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let mut reader = MyReader::new(data);
    let mut buf = [0; 2];

    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }
        println!("{:?}", &buf[..n]);
    }
}
