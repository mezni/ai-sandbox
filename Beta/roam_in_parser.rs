use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{alphanumeric1, char, digit1, multispace0, multispace1, not_line_ending, space1},
    combinator::{map, recognize, opt},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
struct TelecomCommand {
    command_type: String,
    parameters: HashMap<String, String>,
    data_rows: Vec<HashMap<String, String>>,
}

#[derive(Debug)]
struct TelecomFile {
    header: String,
    commands: Vec<TelecomCommand>,
    footer: HashMap<String, String>,
}

fn parse_header(input: &str) -> IResult<&str, &str> {
    terminated(tag("<mgsvp;"), multispace0)(input)
}

fn parse_footer_tag(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, alt((
        tag("TOTNSUB"),
        tag("TOTNSUBA"),
        tag("NSUBPR"),
        tag("NSUBXP"),
        tag("NSUBPXOU"),
        tag("NSUBSGS"),
        tag("NSUBGS"),
        tag("END")
    )))(input)
}

fn parse_footer_value(input: &str) -> IResult<&str, &str> {
    preceded(multispace1, take_while1(|c: char| c.is_ascii_digit()))(input)
}

fn parse_footer_entry(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        parse_footer_tag,
        multispace1,
        take_while1(|c: char| c.is_ascii_digit()),
    )(input)
}

fn parse_command_type(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, alt((tag("ORDERED"), tag("ACT"), tag("MT"))))(input)
}

fn parse_parameter(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        take_while1(|c: char| c.is_ascii_alphabetic()),
        multispace1,
        take_while1(|c: char| !c.is_whitespace() && c != '<'),
    )(input)
}

fn parse_data_row(input: &str) -> IResult<&str, HashMap<String, String>> {
    let (input, values) = separated_list1(
        multispace1,
        take_while1(|c: char| c.is_ascii_digit() || c == '-')
    )(input)?;
    
    let mut row = HashMap::new();
    if values.len() >= 1 {
        row.insert("HLRADDR".to_string(), values[0].to_string());
    }
    if values.len() >= 2 {
        row.insert("NSUB".to_string(), values[1].to_string());
    }
    if values.len() >= 3 {
        row.insert("NSUBA".to_string(), values[2].to_string());
    }
    
    Ok((input, row))
}

fn parse_command_section(input: &str) -> IResult<&str, TelecomCommand> {
    let (input, command_type) = parse_command_type(input)?;
    let (input, params) = many0(terminated(parse_parameter, multispace0))(input)?;
    let (input, _) = opt(tag("\n"))(input)?;
    
    let mut parameters = HashMap::new();
    for (key, value) in params {
        parameters.insert(key.to_string(), value.to_string());
    }

    let (input, data_rows) = many0(terminated(parse_data_row, opt(tag("\n"))))(input)?;
    
    Ok((
        input,
        TelecomCommand {
            command_type: command_type.to_string(),
            parameters,
            data_rows,
        },
    ))
}

fn parse_telecom_file(input: &str) -> IResult<&str, TelecomFile> {
    let (input, header) = parse_header(input)?;
    let (input, _) = multispace0(input)?;
    let (input, commands) = many1(parse_command_section)(input)?;
    let (input, _) = multispace0(input)?;
    
    let (input, footer_entries) = many1(terminated(
        parse_footer_entry,
        opt(tag("\n"))
    )(input)?;
    
    let mut footer = HashMap::new();
    for (key, value) in footer_entries {
        footer.insert(key.to_string(), value.to_string());
    }
    
    let (input, _) = opt(tag("\n"))(input)?;
    let (input, _) = opt(tag("<"))(input)?;

    Ok((
        input,
        TelecomFile {
            header: header.to_string(),
            commands,
            footer,
        },
    ))
}

fn main() {
    let data = r#"<mgsvp;
ORDERED
<
ACT     MSCBC2                    AD-288  TIME 250306 1324  CLUSTER

MT MOBILE SUBSCRIBER SURVEY RESULT

HLRADDR             NSUB       NSUBA
4-8617241450000            2          1
4-48601000297              3          2
4-33191226760011         192        161
4-213661009500           896        723
4-34642200026              7          5 4-436677700101             1          0
4-14054729218              1          1

TOTNSUB
1691532

TOTNSUBA
1593286

NSUBPR
0

NSUBXP
0

NSUBPXOU
1691532

NSUBSGS
1092542

NSUBGS
0

END"#;

    match parse_telecom_file(data) {
        Ok((remaining, file)) => {
            println!("Successfully parsed:");
            println!("Header: {}", file.header);
            println!("\nCommands:");
            for cmd in file.commands {
                println!("{:#?}", cmd);
            }
            println!("\nFooter:");
            for (key, value) in file.footer {
                println!("{}: {}", key, value);
            }
            if !remaining.is_empty() {
                println!("\nRemaining unparsed data: {}", remaining);
            }
        }
        Err(e) => println!("Error parsing: {:?}", e),
    }
}