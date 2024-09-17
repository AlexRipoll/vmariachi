use crate::instruction::Opcode;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map, map_res},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn parse(input: &str) -> IResult<&str, Program> {
        let (input, instructions) = many1(AssemblerInstruction::parse)(input)?;

        Ok((input, Program { instructions }))
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Opcode { opcode: Opcode },
    Register { idx: u8 },
    Operand { value: i32 },
}

impl Token {
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

    fn parse_operand(input: &str) -> IResult<&str, Token> {
        let (input, _) = space0(input)?; // Handle leading whitespace

        let (input, value) = preceded(
            tag("#"),
            map_res(digit1, |digit_str: &str| digit_str.parse::<i32>()),
        )(input)?;

        Ok((input, Token::Operand { value }))
    }
}

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn parse(input: &str) -> IResult<&str, AssemblerInstruction> {
        let (input, (opcode, _, register, _, operand)) = tuple((
            Token::parse_load,     // Parse the opcode
            space0,                // Handle optional spaces
            Token::parse_register, // Parse the register
            space0,                // Handle optional spaces
            Token::parse_operand,  // Parse the integer operand
        ))(input)?;

        Ok((
            input,
            AssemblerInstruction {
                opcode,
                operand1: Some(register),
                operand2: Some(operand),
                operand3: None, // No third operand in this form
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::assembler::{AssemblerInstruction, Program, Token};

    #[test]
    fn test_parse_opcode_load() {
        let input = "load";
        assert_eq!(
            Token::parse_load(input).unwrap(),
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
            Token::parse_register(input).unwrap(),
            ("", Token::Register { idx: 12 },)
        );
    }

    #[test]
    fn test_parse_operand() {
        let input = "#10521";
        assert_eq!(
            Token::parse_operand(input).unwrap(),
            ("", Token::Operand { value: 10521 },)
        );
    }

    #[test]
    fn test_parse_instruction() {
        let parsed = AssemblerInstruction::parse("load $0 #100").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: Token::Opcode {
                        opcode: crate::instruction::Opcode::LOAD
                    },
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: Some(Token::Operand { value: 100 }),
                    operand3: None,
                }
            )
        );
    }

    #[test]
    fn test_parse_program() {
        let parsed = Program::parse("load $0 #100").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                Program {
                    instructions: vec![AssemblerInstruction {
                        opcode: Token::Opcode {
                            opcode: crate::instruction::Opcode::LOAD
                        },
                        operand1: Some(Token::Register { idx: 0 }),
                        operand2: Some(Token::Operand { value: 100 }),
                        operand3: None,
                    }]
                }
            ),
        );
    }
}
