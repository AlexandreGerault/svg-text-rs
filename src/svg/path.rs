use super::bounds::Bounds;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Error;

const POSSIBLE_PATH_CHARS: &[char] = &[
    'M', 'm', 'L', 'l', 'H', 'h', 'V', 'v', 'C', 'c', 'S', 's', 'Q', 'q', 'T', 't', 'A', 'a', 'Z',
    'z',
];

#[derive(Debug)]
struct Command {
    command: char,
    args: Vec<f64>,
}

fn get_commands_from_path_string(path: &str) -> Result<Vec<Command>, Error> {
    lazy_static! {
        static ref COMMANDS_REGEX: Regex = Regex::new(r"([a-zA-Z])([^a-zA-Z]*)").unwrap();
    }

    let mut commands = Vec::new();

    for cap in COMMANDS_REGEX.captures_iter(path) {
        let command = cap.get(1).unwrap().as_str().chars().next().unwrap();
        let args = cap.get(2).unwrap().as_str().to_string();

        if !POSSIBLE_PATH_CHARS.contains(&command) {
            return Err(Error);
        }

        commands.push(Command {
            command,
            args: args
                .split_whitespace()
                .map(|s| s.parse::<f64>().unwrap())
                .collect(),
        });
    }

    Ok(commands)
}

#[derive(Debug)]
pub struct Path {
    commands: Vec<Command>,
}

impl Clone for Command {
    fn clone(&self) -> Self {
        Command {
            command: self.command,
            args: self.args.clone(),
        }
    }
}

impl Path {
    pub fn new(d_attribute: String) -> Result<Self, Error> {
        let commands = get_commands_from_path_string(d_attribute.as_str())?;

        Ok(Path { commands })
    }

    pub fn bounds(&self) -> Result<Bounds, String> {
        let mut bounds = Bounds::new();
        let mut is_first = true;

        for command in &self.commands {
            match command.command {
                'M' => {
                    bounds = bounds.move_last_point(command.args[0], command.args[1], is_first);
                }
                'L' => {
                    bounds = bounds.extends(command.args[0], command.args[1]);
                }
                'l' => {
                    bounds = bounds.extends(
                        bounds.last_point().0 + command.args[0],
                        bounds.last_point().1 + command.args[1],
                    );
                }
                'H' => {
                    bounds = bounds.extends(command.args[0], bounds.last_point().1);
                }
                'h' => {
                    bounds = bounds.extends(
                        bounds.last_point().0 + command.args[0],
                        bounds.last_point().1,
                    );
                }
                'V' => {
                    bounds = bounds.extends(bounds.last_point().0, command.args[0]);
                }
                'v' => {
                    bounds = bounds.extends(
                        bounds.last_point().0,
                        bounds.last_point().1 + command.args[0],
                    );
                }
                'C' => {
                    bounds = bounds.extends(command.args[4], command.args[5]);
                }
                'c' => {
                    bounds = bounds.extends(
                        bounds.last_point().0 + command.args[4],
                        bounds.last_point().1 + command.args[5],
                    );
                }
                'S' => {
                    bounds = bounds.extends(command.args[2], command.args[3]);
                }
                's' => {
                    bounds = bounds.extends(
                        bounds.last_point().0 + command.args[2],
                        bounds.last_point().1 + command.args[3],
                    );
                }
                'Q' => {
                    bounds = bounds.extends(command.args[2], command.args[3]);
                }
                'q' => {
                    bounds = bounds.extends(
                        bounds.last_point().0 + command.args[2],
                        bounds.last_point().1 + command.args[3],
                    );
                }
                'T' => {
                    bounds = bounds.extends(command.args[0], command.args[1]);
                }
                't' => {
                    bounds = bounds.extends(
                        bounds.last_point().0 + command.args[0],
                        bounds.last_point().1 + command.args[1],
                    );
                }
                'A' => {
                    bounds = bounds.extends(command.args[5], command.args[6]);
                }
                'a' => {
                    bounds = bounds.extends(
                        bounds.last_point().0 + command.args[5],
                        bounds.last_point().1 + command.args[6],
                    );
                }
                'Z' | 'z' => {
                    bounds = bounds.close();
                }
                _ => {
                    return Err("Command not implemented".to_string());
                }
            }

            if is_first {
                is_first = false;
            }
        }

        Ok(bounds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_panic_when_having_invalid_svg_path_command() {
        let path = Path::new("M 0 0 W 10 10 20 20".to_string());

        if let Err(_) = path {
            panic!("Invalid path command");
        }
    }

    #[test]
    fn it_parses_parses_attribute() {
        let path = Path::new("M 0 0 L 10 10 20 20".to_string()).unwrap();

        assert_eq!(path.commands.len(), 2);

        assert_eq!(path.commands[0].command, 'M');
        assert_eq!(path.commands[0].args.len(), 2);
        assert_eq!(path.commands[0].args[0], 0.0);
        assert_eq!(path.commands[0].args[1], 0.0);

        assert_eq!(path.commands[1].command, 'L');
        assert_eq!(path.commands[1].args.len(), 4);
        assert_eq!(path.commands[1].args[0], 10.0);
        assert_eq!(path.commands[1].args[1], 10.0);
        assert_eq!(path.commands[1].args[2], 20.0);
        assert_eq!(path.commands[1].args[3], 20.0);
    }

    #[test]
    fn it_extends_a_diagonal_line() {
        let path = Path::new("M 10 20 L 75 100".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0);
        assert_eq!(bounds.y1(), 20.0);
        assert_eq!(bounds.x2(), 75.0);
        assert_eq!(bounds.y2(), 100.0);
    }

    #[test]
    fn it_extends_a_relative_diagonal_line() {
        let path = Path::new("M 10 20 l 65 80".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0);
        assert_eq!(bounds.y1(), 20.0);
        assert_eq!(bounds.x2(), 75.0);
        assert_eq!(bounds.y2(), 100.0);
    }

    #[test]
    fn it_extends_a_horizontal_line() {
        let path = Path::new("M 10 20 H 75".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 75.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 20.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_a_relative_horizontal_line() {
        let path = Path::new("M 10 20 h 65".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 75.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 20.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_a_vertical_line() {
        let path = Path::new("M 10 20 V 75".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 10.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 75.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_a_relative_vertical_line() {
        let path = Path::new("M 10 20 v 55".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 10.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 75.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_a_cubic_bezier_curve() {
        let path = Path::new("M 10 20 C 40 25 25 60 50 50".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 50.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 50.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_a_relative_cubic_bezier_curve() {
        let path = Path::new("M 10 20 c 30 5 15 40 40 30".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 50.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 50.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_a_shortcut_bezier_curve() {
        let path = Path::new("M 10 20 S 25 60 50 50".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 50.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 50.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_a_relative_shortcut_bezier_curve() {
        let path = Path::new("M 10 20 s 15 40 40 30".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 50.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 50.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_a_quadratic_bezier_curve() {
        let path = Path::new("M 10 20 Q 25 60 50 50".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 50.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 50.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_a_relative_bezier_curve() {
        let path = Path::new("M 10 20 q 15 40 40 30".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 50.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 50.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_a_shortcut_quadratic_bezier_curve() {
        let path = Path::new("M 10 20 T 50 50".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 50.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 50.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_a_relative_shortcut_quadratic_bezier_curve() {
        let path = Path::new("M 10 20 t 40 30".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 50.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 50.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_an_arc_curve() {
        let path = Path::new("M 10 20 A 25 25 0 0 0 50 50".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 50.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 50.0, "The second point (y) should change");
    }

    #[test]
    fn it_extends_a_relative_arc_curve() {
        let path = Path::new("M 10 20 a 25 25 0 0 0 40 30".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 50.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 50.0, "The second point (y) should change");
    }

    #[test]
    fn it_close_the_curve() {
        let path = Path::new("M 10 20 L 50 50 Z".to_string()).unwrap();

        let bounds = path.bounds().unwrap();

        assert_eq!(bounds.x1(), 10.0, "The first point (x) shouldn't change");
        assert_eq!(bounds.y1(), 20.0, "The first point (y) shouldn't change");
        assert_eq!(bounds.x2(), 50.0, "The second point (x) should change");
        assert_eq!(bounds.y2(), 50.0, "The second point (y) should change");
        assert_eq!(
            bounds.last_point(),
            (10.0, 20.0),
            "The last point should be the first one"
        );
    }
}
