use std::collections::HashSet;

pub fn count(lines: &[&str]) -> u32 {
    count_rectangles(parse_lines(lines)) as u32
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Line {
    Corner,
    Horizontal,
    Vertical,
}

fn parse_lines(lines: &[&str]) -> HashSet<(usize, usize, Line)> {
    lines
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, c)| match c {
                '+' => Some((x, y, Line::Corner)),
                '|' => Some((x, y, Line::Vertical)),
                '-' => Some((x, y, Line::Horizontal)),
                _ => None,
            })
        })
        .collect()
}

fn defines_rectangle(
    lines: &HashSet<(usize, usize, Line)>,
    top_left: &(usize, usize, Line),
    bot_right: &(usize, usize, Line),
) -> bool {
    top_left.2 == Line::Corner
        && bot_right.2 == Line::Corner
        && lines.contains(&(top_left.0, bot_right.1, Line::Corner))
        && lines.contains(&(bot_right.0, top_left.1, Line::Corner))
        && ((top_left.0 + 1)..bot_right.0).all(|x| {
            (lines.contains(&(x, bot_right.1, Line::Corner))
                || lines.contains(&(x, bot_right.1, Line::Horizontal)))
                && (lines.contains(&(x, top_left.1, Line::Corner))
                    || lines.contains(&(x, top_left.1, Line::Horizontal)))
        })
        && ((top_left.1 + 1)..bot_right.1).all(|y| {
            (lines.contains(&(bot_right.0, y, Line::Corner))
                || lines.contains(&(bot_right.0, y, Line::Vertical)))
                && (lines.contains(&(top_left.0, y, Line::Corner))
                    || lines.contains(&(top_left.0, y, Line::Vertical)))
        })
}

fn count_rectangles(lines: HashSet<(usize, usize, Line)>) -> usize {
    lines
        .iter()
        .map(|top_left| {
            lines
                .iter()
                .filter(|bot_right| {
                    top_left.0 < bot_right.0
                        && top_left.1 < bot_right.1
                        && defines_rectangle(&lines, &top_left, &bot_right)
                })
                .count()
        })
        .sum()
}
