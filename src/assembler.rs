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

    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        self.instructions
            .iter()
            .map(|instruction| instruction.to_bytes()) // Convert each instruction to a Result<Vec<u8>, String>
            .collect::<Result<Vec<_>, _>>() // Collect the results, handling any errors
            .map(|bytes| bytes.into_iter().flatten().collect()) // Fla
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

    fn operand_to_bytes(token: &Option<Token>) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();

        match token {
            Some(Token::Register { idx: n }) => {
                bytes.push(*n);
            }
            Some(Token::Operand { value: n }) => {
                let val = *n as u16;
                let second_byte = val as u8;
                let first_byte = (val >> 8) as u8;
                bytes.push(first_byte);
                bytes.push(second_byte);
            }
            None => {}
            _ => {
                return Err("Opcode found in operand field".to_string());
            }
        }

        Ok(bytes)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let mut bytes: Vec<u8> = Vec::new();

        if let Token::Opcode { opcode: n } = &self.opcode {
            bytes.push(n.clone() as u8);
        } else {
            // TODO: handle error
            return Err("Non-opcode found in opcode field".to_string());
        }

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            let operand_bytes = Self::operand_to_bytes(operand)?;
            bytes.extend_from_slice(&operand_bytes);
        }

        Ok(bytes)
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

    #[test]
    fn test_parse_program_to_bytes() {
        let (_, program) = Program::parse("load $0 #100").unwrap();

        assert_eq!(program.to_bytes().unwrap(), vec![0, 0, 0, 100]);
    }
}
