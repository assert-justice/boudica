
#[derive(Debug, Clone)]
pub struct BoError{
    pub message: String,
    pub char_idx: usize,
}

impl BoError {
    pub fn pretty_print(&self, src: &str){
        let mut line_num = 1;
        let mut line_start = 0;
        // let mut line_end = 0;
        for (idx, c) in src.char_indices(){
            if c == '\n'{
                line_num += 1;
                line_start = idx;
            }
            if idx == self.char_idx{break;}
        }
        let line = src.lines().nth(line_num-1).unwrap();
        println!("Error on line {}", line_num);
        println!("{}", self.message);
        println!("{}", line);
        let mut ptr = String::new();
        for _ in line_start..self.char_idx{
            ptr.push('-');
        }
        ptr.push('^');
        while ptr.len() < line.len() {
            ptr.push('-');
        }
        println!("{}", ptr);
    }
}