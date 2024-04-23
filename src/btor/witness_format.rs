use nom::{branch, bytes::complete, character, combinator, multi, sequence};

use super::{assignment::Assignment, helpers};

#[derive(Debug, Clone)]
pub enum PropKind {
    Bad,
    Justice,
}

#[derive(Debug, Clone)]
pub struct Prop {
    pub kind: PropKind,
    pub idx: u64,
}

impl std::fmt::Display for Prop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            PropKind::Bad => write!(f, "Bad at ")?,
            PropKind::Justice => write!(f, "Justice at ")?,
        };

        write!(f, "{}", self.idx)
    }
}

impl Prop {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        combinator::map(
            sequence::pair(
                branch::alt((complete::tag("b"), complete::tag("j"))),
                character::complete::digit1,
            ),
            |(kind_str, idx_str): (&str, &str)| {
                let idx = idx_str.parse().expect("digit1 parses only digits.");
                let kind = match kind_str {
                    "b" => PropKind::Bad,
                    "j" => PropKind::Justice,
                    _ => unreachable!("Parser recognizes only 'j' and 'b' as prop kinds."),
                };
                Prop { kind, idx }
            },
        )(input)
    }
}

#[derive(Debug, Clone)]
pub struct WitnessHeader {
    pub props: Vec<Prop>,
}

impl WitnessHeader {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        combinator::map(
            sequence::terminated(
                sequence::preceded(complete::tag("sat\n"), multi::many1(Prop::parse)),
                helpers::newline,
            ),
            |props| WitnessHeader { props },
        )(input)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Model {
    pub assignments: Vec<Assignment>,
}

impl Model {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let comment = |input| {
            combinator::opt(sequence::terminated(helpers::comment, helpers::newline))(input)
        };

        let assignment = combinator::opt(Assignment::parse);

        let model_parser =
            combinator::map_opt(sequence::pair(comment, assignment), |(_, assignment)| {
                assignment
            });

        combinator::map(multi::many1(model_parser), |assignments| Model {
            assignments,
        })(input)
    }
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub step: u64,
    pub model: Model,
}

impl Transition {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        combinator::map(
            sequence::pair(
                sequence::terminated(helpers::uint, helpers::newline),
                combinator::opt(Model::parse),
            ),
            |(step, model)| Transition {
                step,
                model: model.unwrap_or_default(),
            },
        )(input)
    }
}

#[derive(Debug, Clone)]
pub struct WitnessFrame {
    pub state_part: Option<Transition>,
    pub input_part: Transition,
}

impl WitnessFrame {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let part_with_prefix =
            |prefix| sequence::preceded(complete::tag(prefix), Transition::parse);

        let state_part = part_with_prefix("#");
        let input_part = part_with_prefix("@");

        combinator::map(
            sequence::pair(combinator::opt(state_part), input_part),
            |(state_part, input_part)| Self {
                state_part,
                input_part,
            },
        )(input)
    }

    fn parse_multi(input: &str) -> nom::IResult<&str, Vec<Self>> {
        multi::many1(Self::parse)(input)
    }
}

#[derive(Debug, Clone)]
pub struct WitnessFormat {
    pub header: WitnessHeader,
    pub frames: Vec<WitnessFrame>,
}

impl WitnessFormat {
    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        combinator::map(
            sequence::tuple((
                WitnessHeader::parse,
                WitnessFrame::parse_multi,
                complete::tag("."),
                combinator::opt(helpers::newline),
            )),
            |(_header, _frames, _dot, _newline)| WitnessFormat {
                header: _header,
                frames: _frames,
            },
        )(input)
    }
}
