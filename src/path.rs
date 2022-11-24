const POSSIBLE_PATH_CHARS: &[char] = &['M', 'm', 'L', 'l', 'H', 'h', 'V', 'v', 'C', 'c', 'S', 's', 'Q', 'q', 'T', 't', 'A', 'a', 'Z', 'z'];

struct Command {
    command: char,
    args: Vec<f64>,
}

struct Path {
    commands: Vec<Command>,
}

impl Path {
    fn new(d_attribute: String) -> Self {
        let split = d_attribute.split_whitespace();

        let elements = split
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        for (i, element) in elements.iter().enumerate() {
            if !POSSIBLE_PATH_CHARS.contains(&element.chars().next().unwrap()) {
                let number = element.parse::<f64>();
                if number.is_err() {
                    panic!("Invalid path element at index {}: {}", i, element);
                }
            }

            let command = element.chars().next().unwrap();
        }
        
        Path { commands: elements }
    }
}

#[test]
#[should_panic]
fn it_panic_when_having_invalid_svg_path_command() {
    Path::new("M 0 0 W 10 10 20 20".to_string());
}

#[test]
fn it_parses_element_to_path() {
    let path = Path::new("M 0 0 L 10 10 20 20".to_string());

    assert_eq!(path.commands.len(), 2);
}