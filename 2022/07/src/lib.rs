use common::Answer;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, not_line_ending, space1},
    sequence::{terminated, tuple},
    IResult,
};
use std::collections::HashMap;

enum ParseOutput<'a> {
    File(File<'a>),
    Directory(String),
    None,
}

#[derive(Debug, Default)]
struct File<'a> {
    size: usize,
    path: String,
    _pd: std::marker::PhantomData<&'a str>,
}

impl<'a> File<'a> {
    pub fn get_directories(&'a self) -> Vec<&'a str> {
        self.path
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '/')
            .map(|(i, _)| {
                if i == 0 {
                    &self.path[..1]
                } else {
                    &self.path[..i]
                }
            })
            .collect()
    }

    pub fn get_size(&'a self) -> usize {
        self.size
    }
}

fn file<'a>(parent_dir: String) -> impl Fn(&'a str) -> IResult<&'a str, ParseOutput> {
    move |s: &str| {
        let (s, (size, _, filename)) =
            terminated(tuple((parse_usize, space1, not_line_ending)), newline)(s)?;

        let path = match parent_dir.as_ref() {
            "/" => format!("/{}", filename),
            _ => format!("{}/{}", parent_dir, filename),
        };

        Ok((
            s,
            ParseOutput::File(File {
                size,
                path,
                ..Default::default()
            }),
        ))
    }
}

fn dir(s: &str) -> IResult<&str, ParseOutput> {
    let (s, _) = terminated(tuple((tag("dir "), not_line_ending)), newline)(s)?;

    Ok((s, ParseOutput::None))
}

fn ls_command(s: &str) -> IResult<&str, ParseOutput> {
    let (s, _) = terminated(tag("$ ls"), newline)(s)?;

    Ok((s, ParseOutput::None))
}

fn cd_command<'a>(current_dir: String) -> impl Fn(&'a str) -> IResult<&'a str, ParseOutput> {
    move |s: &str| {
        let (s, (_, target_dir)) = terminated(tuple((tag("$ cd "), not_line_ending)), newline)(s)?;

        let mut current_dir = current_dir.to_owned();

        match (current_dir.as_str(), target_dir) {
            (_, "..") => {
                if let Some(pos) = current_dir.rfind('/') {
                    if pos != 0 {
                        current_dir.truncate(pos);
                    } else {
                        current_dir.truncate(1);
                    }
                }
            }
            ("", _) => current_dir.push('/'),
            ("/", _) => current_dir.push_str(target_dir),
            _ => {
                current_dir.push('/');
                current_dir.push_str(target_dir)
            }
        }

        Ok((s, ParseOutput::Directory(current_dir)))
    }
}

fn parse_usize(s: &str) -> IResult<&str, usize> {
    use nom::combinator::map;

    map(digit1, |i: &str| {
        i.parse::<usize>().expect("Invalid number")
    })(s)
}

fn parse(input: &str) -> IResult<&str, Vec<File>> {
    let mut files = vec![];
    let mut current_dir = String::new();
    let mut pos: usize = 0;

    loop {
        let cd1 = current_dir.clone();
        let cd2 = current_dir.clone();
        let (s, output) = alt((file(cd1), cd_command(cd2), ls_command, dir))(&input[pos..])?;
        pos = input.len() - s.len();

        match output {
            ParseOutput::File(f) => files.push(f),
            ParseOutput::Directory(d) => current_dir = d,
            _ => {}
        }

        if s.is_empty() {
            return Ok((s, files));
        }
    }
}

fn parse_directories(s: &str) -> HashMap<String, usize> {
    let (_, files) = parse(s).expect("Failed to parse terminal session");
    let mut directories = HashMap::new();

    files.iter().for_each(|f| {
        f.get_directories()
            .iter()
            .for_each(|d| *directories.entry(d.to_string()).or_default() += f.get_size());
    });

    directories
}

pub fn step1(s: &str) -> Answer {
    let total_less_100k: usize = parse_directories(s)
        .values()
        .filter(|d| *d <= &100_000)
        .sum();

    total_less_100k.into()
}

pub fn step2(s: &str) -> Answer {
    let directories = parse_directories(s);

    let largest_dir_size = directories
        .iter()
        .max_by(|left, right| left.1.cmp(right.1))
        .expect("No maximum directory found")
        .1;

    let fs_free_space = 70_000_000 - largest_dir_size;
    let missing_space = 30_000_000 - fs_free_space;

    let smallest_dir_size = *directories
        .values()
        .filter(|v| *v > &missing_space)
        .min()
        .expect("Couldn't find the smallest dir");

    smallest_dir_size.into()
}
