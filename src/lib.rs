use nom::{named, take, char, eof, take_until, take_until1, take_till, take_till1, take_while, one_of, is_space, preceded, space, line_ending, do_parse, opt, map_res, not_line_ending, many1};

mod parser;


named!(take4, take!(4));


named!(rec_start( &[u8] ) -> char,
    r#char!('>')
);

named!(single_word( &[u8] ) -> &[u8],
    take_till1!(is_space)
);


fn is_newline(chr: u8) -> bool {
    chr == b'\n' || chr == b'\r'
}

named!(newline( &[u8] ) -> &[u8],
    take_while!(is_newline)
);

named!(to_eol( &[u8] ) -> &[u8],
    take_till!(is_newline)
);

named!(header( &[u8] ) -> (&[u8], &[u8]),
    do_parse!(
        rec_start >>
        opt!(space) >>
        name: single_word >>
        space >>
        description: to_eol >>
        opt!(line_ending) >>
        (name, description)
    )
);

named!(seq( &[u8] ) -> Vec<&[u8]>,
    many1!(to_eol)
);

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let f = b"This is a test";
        assert_eq!(take4(&f[..]).unwrap(), (&f[4..], &f[0..4]));
    }

    #[test]
    fn test_is_char() {
        let f = b">This is a test";
        assert_eq!(rec_start(&f[..]).unwrap(), (&f[1..], '>'));
    }

    #[test]
    fn test_single_word() {
        let f = b"this is a test";
        assert_eq!(single_word(&f[..]).unwrap(), (&f[4..], &f[0..4]));
    }

    #[test]
    fn test_to_eol() {
        let f = b"this is a test\n";
        assert_eq!(to_eol(&f[..]).unwrap(), (&b"\n"[..], &b"this is a test"[..]));
    }

    #[test]
    fn test_header() {
        let f = b">This is a test\n";
        assert_eq!(header(&f[..]), Ok( (&b""[..], (&b"This"[..], &b"is a test"[..]))));
    }

    #[test]
    fn test_seq() {
        let f = b"AAAA\nBBBBBB\n";
        assert_eq!(seq(&f[..]), Ok((&b""[..], vec![&b""[..]])))
    }
}
