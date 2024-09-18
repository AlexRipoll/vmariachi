use crate::instruction::Opcode;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1, multispace0, space0},
    combinator::{map, map_res, opt},
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
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
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

    fn parse_directive(input: &str) -> IResult<&str, Token> {
        // Parse the directive that starts with a dot `.` followed by an alphanumeric name
        map(preceded(tag("."), alpha1), |name: &str| Token::Directive {
            name: name.to_string(),
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

    fn parse_label_declaration(input: &str) -> IResult<&str, Token> {
        let (input, (name, _, _)) = tuple((
            alphanumeric1, // Parse the label name (alphanumeric string)
            tag(":"),      // Parse the colon `:`
            opt(space0),   // Optionally handle whitespace after the colon
        ))(input)?;

        Ok((
            input,
            Token::LabelDeclaration {
                name: name.to_string(),
            },
        ))
    }

    pub fn parse_label_usage(input: &str) -> IResult<&str, Token> {
        let (input, (_, name, _)) = tuple((
            tag("@"),         // Expect "@" symbol
            alphanumeric1,    // Parse the label name (alphanumeric)
            opt(multispace0), // Optionally allow spaces after the label
        ))(input)?;

        Ok((
            input,
            Token::LabelUsage {
                name: name.to_string(),
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Option<Token>,
    label: Option<Token>,
    directive: Option<Token>,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn parse(input: &str) -> IResult<&str, AssemblerInstruction> {
        // Use the `alt` combinator to try parsing parse_complete or opcode_only (set more
        // restrictive first)
        alt((
            AssemblerInstruction::parse_opcode,
            AssemblerInstruction::parse_directive,
        ))(input)
    }

    fn parse_opcode(input: &str) -> IResult<&str, AssemblerInstruction> {
        let (input, (label, opcode, operand1, operand2, operand3)) = tuple((
            opt(Token::parse_label_declaration), // Optional label declaration
            Token::parse_opcode,                 // Parse the opcode
            opt(AssemblerInstruction::parse_operand), // Optional operand1
            opt(AssemblerInstruction::parse_operand), // Optional operand2
            opt(AssemblerInstruction::parse_operand), // Optional operand3
        ))(input)?;

        Ok((
            input,
            AssemblerInstruction {
                opcode: Some(opcode),
                label,
                directive: None,
                operand1,
                operand2,
                operand3,
            },
        ))
    }

    fn parse_directive(input: &str) -> IResult<&str, AssemblerInstruction> {
        let (input, (label, directive, operand1, operand2, operand3)) = tuple((
            opt(Token::parse_label_declaration), // Optional label declaration
            Token::parse_directive,              // Parse the directive
            opt(AssemblerInstruction::parse_operand), // Optional operand1
            opt(AssemblerInstruction::parse_operand), // Optional operand2
            opt(AssemblerInstruction::parse_operand), // Optional operand3
        ))(input)?;

        Ok((
            input,
            AssemblerInstruction {
                opcode: None,
                directive: Some(directive),
                label,
                operand1,
                operand2,
                operand3,
            },
        ))
    }

    fn parse_operand(input: &str) -> IResult<&str, Token> {
        alt((Token::parse_operand, Token::parse_register))(input)
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

        if let Some(Token::Opcode { opcode: n }) = &self.opcode {
            bytes.push(n.clone() as u8);
        } else {
            return Err("Non-opcode found in opcode field".to_string());
        }

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            let operand_bytes = Self::operand_to_bytes(operand)?;
            bytes.extend_from_slice(&operand_bytes);
        }

        while bytes.len() < 4 {
            bytes.push(0);
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
    fn test_parse_directive() {
        let input = ".data";
        assert_eq!(
            Token::parse_directive(input).unwrap(),
            (
                "",
                Token::Directive {
                    name: "data".to_string()
                }
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
    fn test_parse_label_declaration() {
        let input = "label1:";
        assert_eq!(
            Token::parse_label_declaration(input).unwrap(),
            (
                "",
                Token::LabelDeclaration {
                    name: "label1".to_string()
                }
            ),
        );
    }

    #[test]
    fn test_parse_label_usage() {
        let input = "@label1";
        assert_eq!(
            Token::parse_label_usage(input).unwrap(),
            (
                "",
                Token::LabelUsage {
                    name: "label1".to_string()
                }
            ),
        );
    }

    #[test]
    fn test_parse_instruction() {
        let parsed = AssemblerInstruction::parse_opcode("load $0 #100").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Opcode {
                        opcode: crate::instruction::Opcode::LOAD
                    }),
                    label: None,
                    directive: None,
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: Some(Token::Operand { value: 100 }),
                    operand3: None,
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_with_opcode_and_one_registers() {
        let parsed = AssemblerInstruction::parse_opcode("JMP $0").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Opcode {
                        opcode: crate::instruction::Opcode::JMP
                    }),
                    label: None,
                    directive: None,
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: None,
                    operand3: None,
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_with_opcode_and_two_registers() {
        let parsed = AssemblerInstruction::parse_opcode("LT $0 $2").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Opcode {
                        opcode: crate::instruction::Opcode::LT
                    }),
                    label: None,
                    directive: None,
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: Some(Token::Register { idx: 2 }),
                    operand3: None,
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_with_opcode_and_three_registers() {
        let parsed = AssemblerInstruction::parse_opcode("ADD $0 $2 $3").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Opcode {
                        opcode: crate::instruction::Opcode::ADD
                    }),
                    label: None,
                    directive: None,
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: Some(Token::Register { idx: 2 }),
                    operand3: Some(Token::Register { idx: 3 }),
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_with_opcode_and_three_registers_and_label() {
        let parsed = AssemblerInstruction::parse_opcode("mem1: ADD $0 $2 $3").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Opcode {
                        opcode: crate::instruction::Opcode::ADD
                    }),
                    label: Some(Token::LabelDeclaration {
                        name: "mem1".to_string()
                    }),
                    directive: None,
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: Some(Token::Register { idx: 2 }),
                    operand3: Some(Token::Register { idx: 3 }),
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_with_directive_and_no_operands() {
        let parsed = AssemblerInstruction::parse_directive(".data").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: None,
                    label: None,
                    directive: Some(Token::Directive {
                        name: "data".to_string()
                    }),
                    operand1: None,
                    operand2: None,
                    operand3: None,
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_with_directive_and_one_registers() {
        let parsed = AssemblerInstruction::parse_directive(".data $0").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: None,
                    label: None,
                    directive: Some(Token::Directive {
                        name: "data".to_string()
                    }),
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: None,
                    operand3: None,
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_with_directive_and_two_registers() {
        let parsed = AssemblerInstruction::parse_directive(".data $0 $1").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: None,
                    label: None,
                    directive: Some(Token::Directive {
                        name: "data".to_string()
                    }),
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: Some(Token::Register { idx: 1 }),
                    operand3: None,
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_with_directive_and_three_registers() {
        let parsed = AssemblerInstruction::parse_directive(".data $0 $1 $2").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: None,
                    label: None,
                    directive: Some(Token::Directive {
                        name: "data".to_string()
                    }),
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: Some(Token::Register { idx: 1 }),
                    operand3: Some(Token::Register { idx: 2 }),
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_with_directive_and_three_registers_and_label_declaration() {
        let parsed = AssemblerInstruction::parse_directive("mem1: .data $0 $1 $2").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                AssemblerInstruction {
                    opcode: None,
                    label: Some(Token::LabelDeclaration {
                        name: "mem1".to_string()
                    }),
                    directive: Some(Token::Directive {
                        name: "data".to_string()
                    }),
                    operand1: Some(Token::Register { idx: 0 }),
                    operand2: Some(Token::Register { idx: 1 }),
                    operand3: Some(Token::Register { idx: 2 }),
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
                        opcode: Some(Token::Opcode {
                            opcode: crate::instruction::Opcode::LOAD
                        }),
                        label: None,
                        directive: None,
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
                        opcode: Some(Token::Opcode {
                            opcode: crate::instruction::Opcode::HLT
                        }),
                        label: None,
                        directive: None,
                        operand1: None,
                        operand2: None,
                        operand3: None,
                    }]
                }
            ),
        );
    }

    #[test]
    fn test_parse_program_directive_only_instruction() {
        let parsed = Program::parse(".data").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                Program {
                    instructions: vec![AssemblerInstruction {
                        opcode: None,
                        label: None,
                        directive: Some(Token::Directive {
                            name: "data".to_string()
                        }),
                        operand1: None,
                        operand2: None,
                        operand3: None,
                    }]
                }
            ),
        );
    }

    #[test]
    fn test_parse_program_directive_and_operands_instruction() {
        let parsed = Program::parse(".data $0 $1").unwrap();
        assert_eq!(
            parsed,
            (
                "",
                Program {
                    instructions: vec![AssemblerInstruction {
                        opcode: None,
                        label: None,
                        directive: Some(Token::Directive {
                            name: "data".to_string()
                        }),
                        operand1: Some(Token::Register { idx: 0 }),
                        operand2: Some(Token::Register { idx: 1 }),
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

    #[test]
    fn test_parse_program_to_bytes_jmp() {
        let (_, program) = Program::parse("JMP $1").unwrap();

        assert_eq!(program.to_bytes().unwrap(), vec![6, 1, 0, 0]);
    }

    #[test]
    fn test_parse_program_to_bytes_add() {
        let (_, program) = Program::parse("ADD $0 $3 $1").unwrap();

        assert_eq!(program.to_bytes().unwrap(), vec![1, 0, 3, 1]);
    }
}
