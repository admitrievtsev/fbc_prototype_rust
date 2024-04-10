use std::vec::Vec;
use std::{fs, vec};
use std::string::String;
use std::collections::LinkedList;

macro_rules! mx_csize {
    () => (
        24
    );
}

macro_rules! fixed_chunker_size {
    () => (
        128
    );
}
macro_rules! mn_csize {
    () => (
        7
    );
}

struct DictRecord<> {
    chunk: Vec<char>,
    num: i32,
    size: usize,
}

pub struct Analyser<'anl_lt> {
    f_in: &'anl_lt str,
    f_out: &'anl_lt str,
    dict: Vec<DictRecord<>>,
    cid_list : Vec<i32>,
    chunk_list : Vec<Vec<char>>,
}
impl<'anl_lt> Analyser<'anl_lt>{
    pub fn new(file_in: &'anl_lt str, file_out: &'anl_lt str) -> Option<Analyser<'anl_lt>> {
        Some(Analyser{f_in: file_in, f_out: file_out, dict: vec![], cid_list: vec![], chunk_list: vec![]})
    }
    fn make_dict(&mut self, char_ar: Vec<char>){
        let K: usize = char_ar.len();
        let mut tmparr: Vec<Vec<char>> = vec![vec![]; mx_csize!() - mn_csize!()];
        for i in mn_csize!() .. mx_csize!(){
            for j in 0 .. i + 1{
                tmparr[i - mn_csize!()].push(char_ar[j]);
            }
        }
        for i in 1 .. K - mx_csize!() {
            for j in mn_csize!() .. mx_csize!() {
                for w in 1 .. j + 1 {
                    tmparr[j - mn_csize!()][w - 1] = tmparr[j - mn_csize!()][w]
                }
                tmparr[j - mn_csize!()][j] = char_ar[i + j];
                self.contains_chunk(tmparr[j - mn_csize!()].clone(), j + 1);
            }
        }
        //self.reset_unfrequent_chunks(1);
    }
    fn reset_unfrequent_chunks(&mut self, lower_edge: i32){
        if self.dict.len() == 0 {return}
        let mut g = self.dict.len() - 1;
        let mut k = 0;
        loop {
            k += 1;
            if k % 1000 == 0 {
                println!("RESET LIST: {}", k);
            }
            if self.dict[g].num <= lower_edge {
                self.dict.remove(g);
            }
            if g > 0 {
                g -= 1
            }
            else {
                break
            }
        }
    }
    fn tostr(word: Vec<char>, i1: usize) -> String{
        let mut x = String::new();
        for i in 0..i1{
            x.push(word[i]);
        }
        return x;
    }

    fn contains_chunk(&mut self, st: Vec<char>, ar_size: usize){
        let k = self.dict.iter();
        let mut g = 0;
        for mut i in k{
            if i.size == ar_size{
                let mut fg = true;
                for j in 0 .. ar_size{
                    if i.chunk[j] != st[j] {
                        fg = false;
                        break
                    }
                }
                if fg {
                    self.dict[g].num += 1;
                    return
                }
            }
            g += 1;
        }
        self.dict.push(DictRecord{chunk: st, num: 1, size: ar_size});
        return
    }
    pub fn deduplication(&mut self){
        self.simple_dedup(fixed_chunker_size!());
        self.fbc_dedup();
        let mut ksiz = 0;
        for i in 0 .. self.chunk_list.len(){
            ksiz += self.chunk_list[i].len()
        }
        let contents = fs::read_to_string(self.f_in)
            .expect("Should have been able to read the file");
        let K = contents.len();

        println!("deduplicated {} original {}", self.count_size(), K)
        //Ищем чанки из dict, проводим дедупликацию, возвращаем массив id и массив чанков
        //Запись в f_out
    }
    fn fbc_dedup(&mut self){

        for rec_id in 0 .. self.dict.len(){
            print!(" FIRST CHUNK IS {} OF lEN {}  ", Analyser::<'anl_lt>::tostr(self.dict[rec_id].chunk.clone(), self.dict[rec_id].chunk.len()), self.dict[rec_id].chunk.len());
            for i in 0 ..  self.chunk_list.len(){
                if self.dict[rec_id].chunk.len() <= self.chunk_list[i].len(){
                    for j in  0 .. self.chunk_list[i].len() - self.dict[rec_id].chunk.len(){
                        let mut fg = true;
                        //println!("{}", self.dict[rec_id].size);

                        for w in 0 .. self.dict[rec_id].chunk.len(){
                            if self.dict[rec_id].chunk[w] != self.chunk_list[i][j + w] {
                                fg = false;
                                break
                            }
                        }
                        if fg{
                            print!(" CHUNK FOUND IN {} ", Analyser::<'anl_lt>::tostr(self.chunk_list[i].clone(), self.chunk_list[i].len()));
                            let cutted = self.chunk_list.len();
                            let mut is_found = false;
                            let mut cut_out = self.chunk_list.len();

                            for w in 0 .. self.chunk_list.len(){
                                if self.chunk_list[w] == self.dict[rec_id].chunk{
                                    is_found = true;
                                    cut_out = w;
                                    break
                                }
                            }
                            if cut_out == self.chunk_list.len(){
                                cut_out += 1;
                            }
                            if j == 0{
                                if is_found{

                                    print!(" T1 ");
                                    self.chunk_list[i] = self.chunk_list[i][self.dict[rec_id].chunk.len() ..self.chunk_list[i].len()].to_owned();
                                }
                                else {
                                    print!(" T2 ");
                                    //self.chunk_list.push(self.chunk_list[i][self.dict[rec_id].chunk.len() ..self.chunk_list[i].len()].to_owned());
                                    self.chunk_list[i] = self.dict[rec_id].chunk.clone();
                                }
                            }
                            else{
                                if is_found {
                                    print!(" T3 ");

                                    self.chunk_list.push(self.chunk_list[i][self.dict[rec_id].size + j ..self.chunk_list[i].len()].to_owned());
                                    self.chunk_list[i] = self.chunk_list[i][0..j].to_owned();
                                    //self.chunk_list[i] = self.dict[rec_id].chunk.clone();
                                }
                                else {

                                    self.chunk_list.push(self.chunk_list[i][self.dict[rec_id].size + j ..self.chunk_list[i].len()].to_owned());
                                    self.chunk_list[i] = self.chunk_list[i][0..j].to_owned();
                                    //self.chunk_list.push(self.chunk_list[i][self.dict[rec_id].chunk.len() + j ..self.chunk_list[i].len()].to_owned());
                                    self.chunk_list.push(self.dict[rec_id].chunk.clone());
                                }
                            }
                            print!("deduplicated {} ", self.count_size());
                            break
                        }
                    }
                }
            }
            println!()
        }
    }

    fn count_size(&self) -> usize{
        let mut y : usize = 0;
        for i in &self.chunk_list{
            y += i.len();
        }
        return y;
    }

    fn simple_dedup(&mut self, fix_csize: i32){
        let contents = fs::read_to_string(self.f_in)
            .expect("Should have been able to read the file");
        let K = contents.len();
        let mut char_ar : Vec<char> = vec![' '; K];
        for i in 0..K{
            char_ar[i] = contents.as_bytes()[i] as char;
        }
        let mut i = 0;
        let mut chunk_num = 0;
        while i < K{
            if i % fixed_chunker_size!() == 0{
                chunk_num += 1;
                self.chunk_list.push(vec![]);
                self.cid_list.push((chunk_num - 1) as i32);
            }
            self.chunk_list[chunk_num - 1].push(char_ar[i]);
            i += 1;
        }
        for i in 0 .. self.chunk_list.len(){
            self.make_dict(self.chunk_list[i].clone());
            println!("CHUNK NO {}",  i)
        }
        self.reset_unfrequent_chunks(2);
        let k = self.dict.iter();
        let mut y = 0;
        for i in k{
            if i.num > 1 {
                y += 1;
                println!("{:?} {} {}", Analyser::<'anl_lt>::tostr(i.chunk.clone(), i.size), i.num, i.size)
            }
        }
        println!("{} {}", self.dict.len(), y);
    }
    fn change_chunk(&mut self){
        self.chunk_list = self.chunk_list[mn_csize!()..self.chunk_list.len()].to_owned();
        //self.chunk_list.push(vec![' '; mx_csize!()]);
    }
}

