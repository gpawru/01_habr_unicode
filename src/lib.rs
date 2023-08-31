use v1::get_utf8_sequence_width;

pub mod v1;
pub mod v2;

/// ошибка валидации
#[derive(Debug)]
pub struct Utf8ValidationError
{
    pub valid_up_to: usize,
}

/// ошибка кодирования / декодирования
#[derive(Debug)]
pub enum Utf8EncodeError
{
    OutOfRange,
    SurrogatePoint,
    InvalidBytesCount,
}

/// кодпоинт Unicode из последовательности UTF-8
#[macro_export]
macro_rules! compose_charcode {
    ($c0:expr) => {
        $c0 as u32
    };
    ($c0:expr, $c1:expr) => {
        ((($c0 & 0x1Fu8) as u32) << 6) | ($c1 & 0x3Fu8) as u32
    };
    ($c0:expr, $c1:expr, $c2:expr) => {
        ((($c0 & 0x0Fu8) as u32) << 12) | ((($c1 & 0x3Fu8) as u32) << 6) | ($c2 & 0x3Fu8) as u32
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr) => {
        ((($c0 & 0x07u8) as u32) << 18)
            | ((($c1 & 0x3Fu8) as u32) << 12)
            | ((($c2 & 0x3Fu8) as u32) << 6)
            | (($c3 & 0x3Fu8) as u32)
    };
}

/// декодировать последовательность UTF-8
pub fn decode_utf8(c: Vec<u8>) -> Result<u32, Utf8EncodeError>
{
    if c.is_empty() || (get_utf8_sequence_width(c[0]) as usize != c.len()) {
        return Err(Utf8EncodeError::InvalidBytesCount);
    }

    let codepoint = match c.len() {
        1 => compose_charcode!(c[0]),
        2 => compose_charcode!(c[0], c[1]),
        3 => compose_charcode!(c[0], c[1], c[2]),
        4 => compose_charcode!(c[0], c[1], c[2], c[3]),
        _ => return Err(Utf8EncodeError::InvalidBytesCount),
    };

    if codepoint > 0x10FFFF {
        return Err(Utf8EncodeError::OutOfRange);
    }

    if (0xD800 ..= 0xDFFF).contains(&codepoint) {
        return Err(Utf8EncodeError::SurrogatePoint);
    }

    Ok(codepoint)
}

/// записать кодпоинт в UTF-8
pub fn encode_utf8(c: u32) -> Result<Vec<u8>, Utf8EncodeError>
{
    Ok(match c {
        0 ..= 0x7F => vec![c as u8],
        0x80 ..= 0x07FF => vec![0xC0 | ((c >> 6) as u8), 0x80 | (c as u8 & 0x3F)],
        (0x800 ..= 0xD7FF) | (0xE000 ..= 0xFFFF) => {
            vec![
                0xE0 | ((c >> 12) as u8),
                0x80 | ((c >> 6) as u8 & 0x3F),
                0x80 | (c as u8 & 0x3F),
            ]
        }
        0xD800 ..= 0xDFFF => return Err(Utf8EncodeError::SurrogatePoint),
        0x10000 ..= 0x10FFFF => {
            vec![
                0xF0 | ((c >> 18) as u8),
                0x80 | ((c >> 12) as u8 & 0x3F),
                0x80 | ((c >> 6) as u8 & 0x3F),
                0x80 | (c as u8 & 0x3F),
            ]
        }
        _ => return Err(Utf8EncodeError::OutOfRange),
    })
}

#[cfg(test)]
mod tests;
