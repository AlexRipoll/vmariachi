use crate::instruction::Opcode;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map, map_res},
    sequence::preceded,
    IResult,
};

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

fn parse_register(input: &str) -> IResult<&str, Token> {
    let (input, _) = space0(input)?; // Handle leading whitespace

    let (input, reg_num) = preceded(
        tag("$"),
        map_res(digit1, |digit_str: &str| digit_str.parse::<u8>()),
    )(input)?;

    Ok((input, Token::Register { idx: reg_num }))
}

#[cfg(test)]
mod test {
    use crate::assembler::{parse_load, parse_register, Token};

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

    #[test]
    fn test_parse_register() {
        let input = "$12";
        assert_eq!(
            parse_register(input).unwrap(),
            ("", Token::Register { idx: 12 },)
        );
    }
}
