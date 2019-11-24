extern crate nom;
extern crate strum; // traits
#[macro_use]
extern crate strum_macros;

use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{escaped, take_till, take_while},
    character::complete::{alphanumeric1 as alphanumeric, char, one_of},
    combinator::{cut, map, not, opt},
    error::context,
    multi::many1,
    number::complete::double,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, EnumString)]
pub enum Opcode {
    ADD,
    SUB,

    MUL,
    DIV,

    JMP,
    JNE,
    JE,
    JLT,
    JGT,
    JNZ,

    FCALL,
    ECHO,

    PUSH,
    POP,

    FETCH_READ,
    FETCH_WRITE,
}

#[derive(Debug)]
pub enum Argument {
    Literal(f64),
    String(String),
    Register(String),
    Label(String),
    Reference(String),
}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    arg1: Option<Argument>,
    arg2: Option<Argument>,
    //    target: Option<Argument>,
}

fn space(i: &str) -> IResult<&str, &str> {
    take_while(move |c| " \t\r".contains(c))(i)
}

fn opcode(i: &str) -> IResult<&str, Opcode> {
    let (i, op) = take_while(move |c: char| c.is_uppercase() && c.is_alphabetic())(i)?;
    match Opcode::from_str(&op) {
        Ok(op) => Ok((i, op)),
        Err(_e) => Err(nom::Err::Error((i, nom::error::ErrorKind::ParseTo))), // todo: map and actually parse?
    }
}

fn label(i: &str) -> IResult<&str, Argument> {
    let (i, name) = preceded(char(':'), take_while(move |c: char| !c.is_whitespace()))(i)?;
    Ok((i, Argument::Label(name.to_owned())))
}

fn literal(i: &str) -> IResult<&str, Argument> {
    let (i, value) = alt((
        map(double, |d| Argument::Literal(d)),
        map(string, |s| Argument::String(String::from(s))),
    ))(i)?;
    Ok((i, value))
}

fn string(i: &str) -> IResult<&str, String> {
    let (i, s) = context(
        "string",
        preceded(
            char('\"'),
            cut(terminated(
                escaped(alphanumeric, '\\', one_of("\"n\\")),
                char('\"'),
            )),
        ),
    )(i)?;

    Ok((i, s.to_owned()))
}

fn register(i: &str) -> IResult<&str, Argument> {
    let (i, name) = preceded(opt(space), alphanumeric)(i)?;
    Ok((i, Argument::Register(name.to_owned())))
}

fn reference(i: &str) -> IResult<&str, Argument> {
    let (i, name) = terminated(preceded(char('['), alphanumeric), char(']'))(i)?;
    Ok((i, Argument::Reference(name.to_owned())))
}

fn arg(i: &str) -> IResult<&str, Argument> {
    preceded(space, alt((label, reference, literal, register)))(i)
}

fn sep(i: &str) -> IResult<&str, char> {
    preceded(opt(space), char('\n'))(i)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, (op, arg1, arg2, _)) = tuple((opcode, opt(arg), opt(arg), opt(sep)))(i)?;

    Ok((
        i,
        Instruction {
            opcode: op,
            arg1: arg1,
            arg2: arg2,
        },
    ))
}

fn main() {
    let input = include_str!("../example1.masm");
    let (input, instr1) = instruction(&input).unwrap();
    let (input, instr2) = instruction(&input).unwrap();

    println!("{:?}", instr1);
    println!("{:?}", instr2);
}
