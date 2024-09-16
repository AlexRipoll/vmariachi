use crate::instruction::Opcode;
use nom::{bytes::complete::tag, combinator::map, IResult};

#[derive(Debug, PartialEq)]
pub enum Token {
    Opcode { opcode: Opcode },
    Register { idx: u8 },
}

fn parse_load(input: &str) -> IResult<&str, Token> {
    map(tag("load"), |_| Token::Opcode {
        opcode: Opcode::LOAD,
    })(input)
}

#[cfg(test)]
mod test {
    use crate::assembler::{parse_load, Token};

    #[test]
    fn test_parse_opcode_load() {
        let input = "load";
        assert_eq!(
            parse_load(input).unwrap(),
            (
                "",
                Token::Opcode {
                    opcode: crate::instruction::Opcode::LOAD,
                },
            )
        );
    }
}
