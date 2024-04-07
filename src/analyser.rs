use std::vec::Vec;
use std::fs;
use std::string::String;

macro_rules! mx_csize {
    () => (
        24
    );
}
macro_rules! mn_csize {
    () => (
        7
    );
}

struct DictRecord<> {
    chunk: [char; mx_csize!()],
    num: i32,
    size: usize,
}

pub struct Analyser<'anl_lt> {
    f_in: &'anl_lt str,
    f_out: &'anl_lt str,
    dict: Vec<DictRecord<>>,
}
impl<'anl_lt> Analyser<'anl_lt>{
    pub fn new(file_in: &'anl_lt str, file_out: &'anl_lt str) -> Option<Analyser<'anl_lt>> {
        Some(Analyser{f_in: file_in, f_out: file_out, dict: vec![]})
    }
    fn make_dict(&mut self){
        let contents = fs::read_to_string(self.f_in)
            .expect("Should have been able to read the file");
        const K: usize = 16000;
        let mut char_ar : [char; K] = ['a'; K];
        for i in 0..K{
            char_ar[i] = contents.as_bytes()[i] as char;
        }
        let mut tmparr: [[char; mx_csize!()]; mx_csize!() - mn_csize!()] = [[' '; mx_csize!()]; mx_csize!() - mn_csize!()];
        for i in mn_csize!() .. mx_csize!(){
            for j in 0 .. i + 1{
                tmparr[i - mn_csize!()][j] = char_ar[j]
            }
        }
        for i in 1 .. K - mx_csize!() {
            if i % 6000 == 0{
                self.reset_unfrequent_chunks();
            }
            for j in mn_csize!() .. mx_csize!() {
                for w in 1 .. j + 1 {
                    tmparr[j - mn_csize!()][w - 1] = tmparr[j - mn_csize!()][w]
                }
                tmparr[j - mn_csize!()][j] = char_ar[i + j];
                self.contains_chunk(tmparr[j - mn_csize!()], j + 1);
            }
            if(i % 100) == 0{
                println!("{}", i)
            }
        }
        self.reset_unfrequent_chunks();
        let k = self.dict.iter();
        for i in k{
            if i.num > 8 {
                println!("{:?} {} {}", Analyser::<'anl_lt>::tostr(i.chunk, i.size), i.num, i.size)
            }
        }
    }
    fn reset_unfrequent_chunks(&mut self){
        let mut g = self.dict.len() - 1;
        let mut k = 0;

        loop {
            k += 1;
            if k % 1000 == 0 {
                println!("RESET LIST: {}", k);
            }
            if self.dict[g].num == 1 {
                self.dict.remove(g);
            }
            if g > 0 {
                g -= 1
            } else { break }
        }
    }
    fn tostr(word: [char; mx_csize!()], i1: usize) -> String{
        let mut x = String::new();
        for i in 0..i1{
            x.push(word[i]);
        }
        return x;
    }
    fn contains_chunk(&mut self, st: [char; mx_csize!()], ar_size: usize){
        let k = self.dict.iter();
        let mut g = 0;
        for mut i in k{
            if i.size == ar_size{
                let mut fg = true;
                for j in 0 .. ar_size{
                    if i.chunk[j] != st[j] {
                        fg = false;
                        /*
                        if (Analyser::<'anl_lt>::tostr(st, i.size) == "Minnesota") {
                            //println!("ARG: {} DICT: {}", Analyser::<'anl_lt>::tostr(st, i.size), Analyser::<'anl_lt>::tostr(i.chunk, i.size));
                            //println!("DIFF: {} != {}", i.chunk[j], st[j]);
                        }
                         */
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
        self.make_dict();
        //Ищем чанки из dict, проводим дедупликацию, возвращаем массив id и массив чанков
        //Запись в f_out
    }
}

