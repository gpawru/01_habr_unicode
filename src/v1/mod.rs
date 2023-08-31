use crate::Utf8ValidationError;

/// если данные являются валидной строкой UTF-8 - преобразуем их в строковый слайс
#[inline(never)]
pub fn from_utf8(source: &[u8]) -> Result<&str, Utf8ValidationError>
{
    match validate_utf8(source) {
        Ok(_) => {
            #[allow(clippy::transmute_bytes_to_str)]
            // совершенно то же самое выполняет функция core::mem::from_utf8_unchecked,
            // но я не использую её, чтобы показать, что под капотом
            //
            // SAFETY: это безопасно, потому-что и слайс u8, и строковый слайс имеют одинаковый лейаут
            Ok(unsafe { core::mem::transmute(source) })
        }
        Err(error) => {
            // здесь, при желании, можно написать код, который получит максимальную последующую длину
            // невалидной последовательности байт
            Err(error)
        }
    }
}

/// валидация UTF-8. при ошибке возвращаем длину валидного блока данных
#[inline(always)]
pub fn validate_utf8(source: &[u8]) -> Result<(), Utf8ValidationError>
{
    let mut index = 0;
    let len = source.len();

    while index < len {
        let old_index = index;

        // напишем пару макросов, который облегчат нам код - возврат ошибки и получение следующего байта

        macro_rules! err {
            () => {
                return Err(Utf8ValidationError {
                    valid_up_to: old_index,
                })
            };
        }

        macro_rules! next {
            () => {{
                index += 1;
                // мы читаем последовательность UTF-8, и ожидаем наличие в ней определенного количества байт
                // если же неожиданно их нет - это очевидная ошибка
                if index >= len {
                    err!()
                };
                source[index]
            }};
        }

        let first = source[index];
        let sequence_width = get_utf8_sequence_width(first);

        // проверяем последовательность
        match sequence_width {
            1 => (),
            2 => {
                if !(0x80 ..= 0xBF).contains(&next!()) {
                    err!()
                }
            }
            3 => {
                match (first, next!()) {
                    (0xE0, 0xA0 ..= 0xBF)
                    | (0xE1 ..= 0xEC, 0x80 ..= 0xBF)
                    | (0xED, 0x80 ..= 0x9F)
                    | (0xEE ..= 0xEF, 0x80 ..= 0xBF) => {}
                    _ => err!(),
                }

                if !(0x80 ..= 0xBF).contains(&next!()) {
                    err!()
                }
            }
            4 => {
                match (first, next!()) {
                    (0xF0, 0x90 ..= 0xBF)
                    | (0xF1 ..= 0xF3, 0x80 ..= 0xBF)
                    | (0xF4, 0x80 ..= 0x8F) => {}
                    _ => err!(),
                }
                if !(0x80 ..= 0xBF).contains(&next!()) {
                    err!()
                }
                if !(0x80 ..= 0xBF).contains(&next!()) {
                    err!()
                }
            }
            _ => err!(),
        }

        index += 1;
    }

    Ok(())
}

/// получаем количество байт в последовательности UTF-8
#[inline(always)]
pub fn get_utf8_sequence_width(first: u8) -> u8
{
    match first {
        0 ..= 0x7F => 1,
        0xC2 ..= 0xDF => 2,
        0xE0 ..= 0xEF => 3,
        0xF0 ..= 0xF4 => 4,
        _ => 0,
    }
}
