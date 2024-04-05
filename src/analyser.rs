use std::collections::HashMap;
use std::vec::Vec;

pub struct Analyser {
    f_in: &str,
    f_out: &str,
    dict: Vec<&str>,
    //TODO
}
impl Analyser {
    fn set(file_in: &str, file_out: &str){
        self.f_in = file_in;
        self.f_out = file_out;
    }
    fn make_dict(hm: HashMap<i32, i32>){
        //Чтение из f_in
        //Перебираем файл, делаем набор частых чанков, пишем их в dict
    }
    fn deduplication(array: &mut [&str; 0]){
        //Ищем чанки из dict, проводим дедупликацию, возвращаем массив id и массив чанков 
        //Запись в f_out
    }
}
