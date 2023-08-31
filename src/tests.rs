use crate::{decode_utf8, encode_utf8};

/// тестируем кодирование / декодирование
#[test]
fn test_encode_decode()
{
    for codepoint in 0 ..= 0x10FFFF {
        let encoded = encode_utf8(codepoint);

        // суррогатная пара
        if (0xD800 ..= 0xDFFF).contains(&codepoint) {
            assert!(encoded.is_err());

            continue;
        }

        let decoded = decode_utf8(encoded.unwrap()).unwrap();
        assert_eq!(codepoint, decoded);
    }
}

/// проверяем все варианты функции валидации разом
#[test]
fn test_validation()
{
    test_cases_with("v1", |bytes: &[u8]| -> bool {
        crate::v1::from_utf8(bytes).is_ok()
    });

    test_cases_with("v2", |bytes: &[u8]| -> bool {
        crate::v2::from_utf8(bytes).is_ok()
    });
}

/// кейс проверки валидации - последовательность и описание, в чём же её суть
struct Case
{
    pub bytes: &'static [u8],
    pub desc: &'static str,
}

/// проверка валидации с выбранной функцией
fn test_cases_with(version: &str, f: fn(&[u8]) -> bool)
{
    for case in ill_formed() {
        assert!(!f(case.bytes), "{}, ожидалось: {}", version, case.desc)
    }

    for case in well_formed() {
        assert!(f(case.bytes), "{}, ожидалось: {}", version, case.desc)
    }
}

/// ill-formed UTF-8 последовательности
fn ill_formed() -> Vec<Case>
{
    vec![
        Case {
            bytes: &[0b_1111_1000, 0b_1000_0000, 0b_1000_0000, 0b_1000_0000],
            desc: "невалидные старшие биты первого байта",
        },
        Case {
            bytes: &[0b_1000_0000, 0b_1000_0000],
            desc: "старшие биты первого байта - 0b10",
        },
        Case {
            bytes: &[0b_1100_0010, 0b_0000_0000],
            desc: "старшие биты во втором байте не равны 0b10",
        },
        Case {
            bytes: &[0b_1110_1111, 0b_1000_0000],
            desc: "отсутствует ожидаемый третий байт последовательности",
        },
        Case {
            bytes: &[0b_1110_1111, 0b_1000_0000, 0b_0000_0000],
            desc: "старшие биты в третьем байте не равны 0b10",
        },
        Case {
            bytes: &[0b_1111_0011, 0b_1000_0000, 0b_1000_0000],
            desc: "отсутствует ожидаемый четвёртый байт последовательности",
        },
        Case {
            bytes: &[0b_1111_0011, 0b_1000_0000, 0b_1000_0000, 0b_0000_0000],
            desc: "старшие биты в четвёртом байте не равны 0b10",
        },
        Case {
            bytes: &[0b_1100_0001, 0b_1011_1111],
            desc: "избыточное кодирование с помощью двухбайтной последовательности",
        },
        Case {
            bytes: &[0b_1110_0000, 0b_1001_1111, 0b_1011_1111],
            desc: "избыточное кодирование с помощью трёхбайтной последовательности",
        },
        Case {
            bytes: &[0b_1111_0000, 0b_1000_1111, 0b_1011_1111, 0b_1011_1111],
            desc: "избыточное кодирование с помощью четырёхбайтной последовательности",
        },
        Case {
            bytes: &[0b_1111_0100, 0b_1001_0000, 0b_1000_0000, 0b_1000_0000],
            desc: "вышли за пределы диапазона Unicode",
        },
        Case {
            bytes: &[0b_1110_1101, 0b_1010_0000, 0b_1000_0000],
            desc: "суррогатные пары",
        },
        Case {
            bytes: &[0b_1110_1101, 0b_1011_1111, 0b_1011_1111],
            desc: "суррогатные пары",
        },
    ]
}

/// well-formed UTF-8 последовательности
fn well_formed() -> Vec<Case>
{
    vec![
        // 1 байт
        Case {
            bytes: &[0x00],
            desc: "1 байт, нижняя граница",
        },
        Case {
            bytes: &[0x7f],
            desc: "1 байт, верхняя граница",
        },
        // 2 байта
        Case {
            bytes: &[0xc2, 0x80],
            desc: "2 байта, нижняя граница",
        },
        Case {
            bytes: &[0xdf, 0xbf],
            desc: "2 байта, верхняя граница",
        },
        // 3 байта
        Case {
            bytes: &[0xe0, 0xa0, 0x80],
            desc: "3 бата, нижняя граница",
        },
        Case {
            bytes: &[0xe0, 0xbf, 0xbf],
            desc: "3 байта",
        },
        Case {
            bytes: &[0xe1, 0x80, 0x80],
            desc: "3 байта",
        },
        Case {
            bytes: &[0xec, 0xbf, 0xbf],
            desc: "3 байта",
        },
        Case {
            bytes: &[0xed, 0x9f, 0xbf],
            desc: "3 байта, далее идут суррогатные пары",
        },
        Case {
            bytes: &[0xee, 0x80, 0x80],
            desc: "3 байта, после суррогатных пар",
        },
        Case {
            bytes: &[0xef, 0xbf, 0xbf],
            desc: "3 байта, верхняя граница",
        },
        // 4 байта
        Case {
            bytes: &[0xf0, 0x90, 0x80, 0x80],
            desc: "4 байта, нижняя граница",
        },
        Case {
            bytes: &[0xf1, 0x80, 0x80, 0x80],
            desc: "4 байта",
        },
        Case {
            bytes: &[0xf3, 0xbf, 0xbf, 0xbf],
            desc: "4 байта",
        },
        Case {
            bytes: &[0xf4, 0x8f, 0xbf, 0xbf],
            desc: "4 байта, верхняя граница",
        },
    ]
}
