#[warn(dead_code)]

pub struct Tester<'tst_lt> {
    f_in: &'tst_lt str,
    f_out: &'tst_lt str,
}


impl<'tst_lt> Tester<'_> {

    /*
    pub fn new(file_in: &'tst_lt str, file_out: &'tst_lt str) -> Option<Tester<'tst_lt>> {
        Some(Tester {
            f_in: file_in,
            f_out: file_out,
        })
    }
    fn test_size() -> bool {
        //Вычисляем размер исходного файла и рамер задедуплицированного и сравниваем их
        //Debuglog
        //Возвращаем результат сравнения
        return true;
    }
    fn test_content() -> bool {
        //Пересобираем задедуплицированный файлик и сравниваем с исходным
        //Debuglog
        //Возвращаем результат сравнения
        return true;
    }

     */
}
