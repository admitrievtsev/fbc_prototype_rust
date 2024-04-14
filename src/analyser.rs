use std::string::String;
use std::vec::Vec;
use std::{fs, vec};

const MX_CSIZE: usize = 24;
const FIXED_CHUNKER_SIZE: usize = 128;
const MN_CSIZE: usize = 7;

macro_rules! inc {
    ($x:expr) => {$x += 1};
}
macro_rules! dec {
    ($x:expr) => {$x -= 1};
}
struct DictRecord {
    chunk: Vec<char>,
    num: i32,
    size: usize,
}

pub struct Analyser {
    dict: Vec<DictRecord>,  // hashmap? chunk_map
    chunk_ids: Vec<i32>,    // hashset?
    chunks: Vec<Vec<char>>, // u8 instead
}

impl Analyser {
    pub fn new() -> Analyser {
        Analyser {
            dict: vec![],
            chunk_ids: vec![],
            chunks: vec![],
        }
    }

    fn make_dict(&mut self, chars: Vec<char>) {
        let mut temp_chunks: Vec<Vec<char>> = vec![vec![]; MX_CSIZE - MN_CSIZE];
        for slice_index in MN_CSIZE..MX_CSIZE {
            for char in chars.iter().take(slice_index + 1) {
                temp_chunks[slice_index - MN_CSIZE].push(*char);
            }
        }
        for start_index in 1 .. chars.len() - MX_CSIZE {
            for chunk_size in MN_CSIZE..MX_CSIZE {
                for char_index in 1..chunk_size + 1 {
                    temp_chunks[chunk_size - MN_CSIZE][char_index - 1] = temp_chunks[chunk_size - MN_CSIZE][char_index]
                }
                temp_chunks[chunk_size - MN_CSIZE][chunk_size] = chars[start_index + chunk_size];
                self.add_chunk(temp_chunks[chunk_size - MN_CSIZE].clone(), chunk_size + 1);
            }
        }
    }

    fn tostr(word: &Vec<char>) -> String {
        let mut temp_str = String::new();
        for char in word.iter() {
            temp_str.push(*char);
        }
        temp_str
    }

    fn add_chunk(&mut self, chunk: Vec<char>, str_size: usize) {
        let mut chunk_dict_id = 0;
        for dict_chunk in self.dict.iter()
        {
            if dict_chunk.size == str_size {
                for char_index in 0 .. str_size + 1 {
                    if char_index == str_size{
                        inc!(self.dict[chunk_dict_id].num);
                        return;
                    }
                    if dict_chunk.chunk[char_index] != chunk[char_index] {
                        break;
                    }
                }
            }
            inc!(chunk_dict_id);
        }
        self.dict.push(DictRecord {
            chunk,
            num: 1,
            size: str_size,
        })
    }

    pub fn deduplicate(&mut self, file_in: &str, file_out: &str) {
        self.simple_dedup(file_in);
        self.fbc_dedup();
        let contents =
            fs::read_to_string(file_in).expect("Should have been able to read the file");
        let k = contents.len();

        println!("deduplicated {} original {}", self.dict_count_size(), k)
    }

    fn fbc_dedup(&mut self) {
        for dict_record in self.dict.iter() {
            print!(
                " FIRST CHUNK IS {} OF lEN {}  ",
                Analyser::tostr(
                    &dict_record.chunk,
                ),
                dict_record.chunk.len()
            );
            for chunk in 0..self.chunks.len() {
                if dict_record.chunk.len() <= self.chunks[chunk].len() {
                    for chunk_char in 0..self.chunks[chunk].len() - dict_record.chunk.len() {
                        let mut is_chunk_correct = true;
                        for char_index in 0..dict_record.chunk.len() {
                            if dict_record.chunk[char_index] != self.chunks[chunk][chunk_char + char_index] {
                                is_chunk_correct = false;
                                break
                            }
                        }

                        if is_chunk_correct {
                            print!(
                                " CHUNK FOUND IN {} ",
                                Analyser::tostr(
                                    &self.chunks[chunk],
                                )
                            );
                            let cutted = self.chunks.len();
                            let mut is_found = false;
                            let mut cut_out = self.chunks.len();

                            for chunk_index in 0 ..self.chunks.len() {
                                if self.chunks[chunk_index] == dict_record.chunk {
                                    is_found = true;
                                    cut_out = chunk_index;
                                    break;
                                }
                            }
                            if cut_out == self.chunks.len() {
                                inc!(cut_out);
                            }
                            if chunk_char == 0 {
                                self.chunks[chunk] = self.chunks[chunk]
                                    [dict_record.chunk.len()..self.chunks[chunk].len()]
                                    .to_owned();
                                if !is_found {
                                    self.chunks.push(dict_record.chunk.clone());
                                }
                            } else {
                                self.chunks.push(self.chunks[chunk][dict_record.size + chunk_char..self.chunks[chunk].len()].to_owned());
                                self.chunks[chunk] = self.chunks[chunk][0..chunk_char].to_owned();
                                if !is_found {
                                    self.chunks.push(dict_record.chunk.clone());
                                }
                            }
                            print!("deduplicated {} ", self.dict_count_size());
                            break;
                        }
                    }
                }
            }
            println!()
        }
    }

    fn dict_count_size(&self) -> usize {
        return self.chunks.iter().fold(0, |acc, x| acc + x.len())
    }

    fn simple_dedup(&mut self, f_in: &str) {
        let contents =
            fs::read_to_string(f_in).expect("Should have been able to read the file");
        let k = contents.len();
        let mut chars: Vec<char> = vec![' '; k];
        for i in 0..k {
            chars[i] = contents.as_bytes()[i] as char;
        }
        let mut i = 0;
        let mut chunk_num = 0;
        while i < k {
            // for
            if i % FIXED_CHUNKER_SIZE == 0 {
                inc!(chunk_num);
                self.chunks.push(vec![]);
                self.chunk_ids.push((chunk_num - 1) as i32);
            }
            self.chunks[chunk_num - 1].push(chars[i]);
            i += 1; // i += FIXED_CHUNKER_SIZE
        }

        for chunk_index in 0..self.chunks.len() {
            self.make_dict(self.chunks[chunk_index].clone());
            println!("CHUNK NO {}", chunk_index)
        }
        //let iter_dict = ;
        self.dict = self.dict.drain(..).filter(|x| x.num > 1).collect();
        let mut y = 0;
        for record in self.dict.iter() {
            if record.num > 1 {
                y += 1;
                println!(
                    "{:?} {} {}",
                    Analyser::tostr(&record.chunk),
                    record.num,
                    record.size
                )
            }
        }
        println!("{} {}", self.dict.len(), y);
    }
}
