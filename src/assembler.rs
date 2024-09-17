use crate::instruction::Opcode;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, space0},
    combinator::{map_res, opt},
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
            .map(|bytes| bytes.into_iter().flatten().collect())
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Opcode { opcode: Opcode },
    Register { idx: u8 },
    Operand { value: i32 },
}

impl Token {
    fn parse_opcode(input: &str) -> IResult<&str, Token> {
        map_res(alpha1, |opcode_str: &str| {
            let lower_opcode = opcode_str.to_lowercase();
            Ok(Token::Opcode {
                opcode: Opcode::from(lower_opcode.as_str()),
            }) as Result<Token, ()>
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
        // Use the `alt` combinator to try parsing parse_complete or opcode_only (set more
        // restrictive first)
        alt((
            AssemblerInstruction::parse_opcode_with_register_and_operand,
            AssemblerInstruction::parse_opcode_with_optional_registers,
            AssemblerInstruction::parse_opcode,
        ))(input)
    }

    fn parse_opcode(input: &str) -> IResult<&str, AssemblerInstruction> {
        let (input, (opcode, _)) = tuple((
            Token::parse_opcode, // Parse the opcode (function that parses the opcode)
            opt(multispace0),    // Optional whitespace
        ))(input)?;

        Ok((
            input,
            AssemblerInstruction {
                opcode,
                operand1: None,
                operand2: None,
                operand3: None,
            },
        ))
    }

    fn parse_opcode_with_optional_registers(input: &str) -> IResult<&str, AssemblerInstruction> {
        let (input, (opcode, _, register1, _, register2, _, register3)) = tuple((
            Token::parse_opcode,   // Parse the opcode
            space0,                // Handle optional spaces
            Token::parse_register, // Parse the register
            space0,                // Handle optional spaces
            opt(Token::parse_register),
            space0, // Handle optional spaces
            opt(Token::parse_register),
        ))(input)?;

        Ok((
            input,
            AssemblerInstruction {
                opcode,
                operand1: Some(register1),
                operand2: register2,
                operand3: register3,
            },
        ))
    }

    fn parse_opcode_with_register_and_operand(input: &str) -> IResult<&str, AssemblerInstruction> {
        let (input, (opcode, _, register, _, operand)) = tuple((
            Token::parse_opcode,   // Parse the opcode
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
            return Err("Non-opcode found in opcode field".to_string());
        }

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
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
            Token::parse_opcode(input).unwrap(),
            (
                "",
                Token::Opcode {
                    opcode: crate::instruction::Opcode::LOAD,
                },
            )
        );
    }

    #[test]
    fn test_parse_opcode_jmp() {
        let input = "jmp";
        assert_eq!(
            Token::parse_opcode(input).unwrap(),
            (
                "",
                Token::Opcode {
                    opcode: crate::instruction::Opcode::JMP,
                },
            )
        );
    }

    #[test]
    fn test_parse_illegal_opcode() {
        let input = "alod";
        assert_eq!(
            Token::parse_opcode(input).unwrap(),
            (
                "",
                Token::Opcode {
                    opcode: crate::instruction::Opcode::IGL,
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
        let parsed =
            AssemblerInstruction::parse_opcode_with_register_and_operand("load $0 #100").unwrap();
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
    fn test_parse_instruction_opcode_only() {
        let parsed = AssemblerInstruction::parse_opcode("hlt").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: Token::Opcode {
                        opcode: crate::instruction::Opcode::HLT
                    },
                    operand1: None,
                    operand2: None,
                    operand3: None,
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_with_one_registers() {
        let parsed = AssemblerInstruction::parse_opcode_with_optional_registers("JMP $0").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: Token::Opcode {
                        opcode: crate::instruction::Opcode::JMP
                    },
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: None,
                    operand3: None,
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_with_two_registers() {
        let parsed =
            AssemblerInstruction::parse_opcode_with_optional_registers("LT $0 $2").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: Token::Opcode {
                        opcode: crate::instruction::Opcode::LT
                    },
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: Some(Token::Register { idx: 2 }),
                    operand3: None,
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_three_registers() {
        let parsed =
            AssemblerInstruction::parse_opcode_with_optional_registers("ADD $0 $2 $3").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: Token::Opcode {
                        opcode: crate::instruction::Opcode::ADD
                    },
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: Some(Token::Register { idx: 2 }),
                    operand3: Some(Token::Register { idx: 3 }),
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
    fn test_parse_program_opcode_only_instruction() {
        let parsed = Program::parse("hlt").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                Program {
                    instructions: vec![AssemblerInstruction {
                        opcode: Token::Opcode {
                            opcode: crate::instruction::Opcode::HLT
                        },
                        operand1: None,
                        operand2: None,
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
