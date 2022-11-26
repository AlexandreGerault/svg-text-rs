use std::char::from_u32;

use html_escape::decode_html_entities;
use roxmltree::*;

use super::path::Path;

const VALID_CHARS: [char; 94] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', 'É', 'È', 'Ê', 'Ë', 'é', 'è', 'ê', 'ë', 'À', 'Â', 'Ä', 'à', 'â', 'ä',
    'Ô', 'Ö', 'ô', 'ö', 'Ù', 'Û', 'Ü', 'ù', 'û', 'ü', 'Ç', 'ç', 'Œ', 'œ', 'Æ', 'æ', 'ß', 'µ',
];

struct Glyph {
    unicode: u32,
    horizontal_advance_x: f64,
    path: String,
}

pub struct Font {
    horizontal_advance_x: f64,
    units_per_em: f64,
    ascent: f64,
    descent: f64,
    glyphs: Vec<Glyph>,
}

impl Font {
    pub fn new(svg: String) -> Result<Self, String> {
        let doc = Document::parse(svg.as_str()).unwrap();

        let font_element = match doc.descendants().find(|n| n.has_tag_name("font")) {
            Some(n) => n,
            None => return Err("No font element found".to_string()),
        };

        let font_face_element = match font_element
            .descendants()
            .find(|n| n.has_tag_name("font-face"))
        {
            Some(n) => n,
            None => return Err("No font-face element found".to_string()),
        };

        let horizontal_advance_x = match font_element.attribute("horiz-adv-x") {
            Some(n) => n.parse::<f64>().unwrap(),
            None => return Err("No horiz-adv-x attribute found".to_string()),
        };

        let units_per_em = match font_face_element.attribute("units-per-em") {
            Some(n) => n.parse::<f64>().unwrap(),
            None => return Err("No units-per-em attribute found".to_string()),
        };

        let ascent = match font_face_element.attribute("ascent") {
            Some(n) => n.parse::<f64>().unwrap(),
            None => return Err("No ascent attribute found".to_string()),
        };

        let descent = match font_face_element.attribute("descent") {
            Some(n) => n.parse::<f64>().unwrap(),
            None => return Err("No descent attribute found".to_string()),
        };

        let glyphs = font_element
            .descendants()
            .filter(|n| {
                if !n.has_tag_name("glyph") {
                    return false;
                }
                
                if !n.attribute("unicode").is_some() {
                    return false;
                }

                if !n.attribute("d").is_some() {
                    return false;
                }

                if VALID_CHARS.contains(&n.attribute("unicode").unwrap().chars().next().unwrap()) {
                    return true;
                }

                false
            })
            .map(|n| {
                let unicode = n.attribute("unicode").unwrap().parse::<String>().unwrap();

                let character = html_escape::decode_html_entities(&unicode)
                    .chars()
                    .next()
                    .unwrap();

                let horizontal_advance_x = match n.attribute("horiz-adv-x") {
                    Some(n) => n.parse::<f64>().unwrap(),
                    None => horizontal_advance_x,
                };

                let path = match n.attribute("d") {
                    Some(n) => n.to_string(),
                    None => "".to_string(),
                };

                Glyph {
                    unicode: character as u32,
                    horizontal_advance_x: horizontal_advance_x,
                    path: String::from(path),
                }
            })
            .collect::<Vec<Glyph>>();

        Ok(Font {
            horizontal_advance_x,
            units_per_em,
            ascent,
            descent,
            glyphs,
        })
    }

    pub fn font_height(&self) -> f64 {
        let mut min: f64 = 0.0;
        let mut max: f64 = 0.0;

        for glyph in &self.glyphs {
            if let Ok(path) = Path::new(glyph.path.clone()) {
                let bounds = path.bounds().unwrap();

                min = min.min(bounds.y1()).min(bounds.y2());
                max = max.max(bounds.y1()).max(bounds.y2());
            }
        }

        max - min
    }

    pub fn text_height(&self, text: String) -> f64 {
        let mut min: f64 = 0.0;
        let mut max: f64 = 0.0;

        for character in text.chars() {
            let glyph = self.glyphs.iter().find(|g| g.unicode == character as u32);

            if let Some(glyph) = glyph {
                if let Ok(path) = Path::new(glyph.path.clone()) {
                    let bounds = path.bounds().unwrap();

                    min = min.min(bounds.y1()).min(bounds.y2());
                    max = max.max(bounds.y1()).max(bounds.y2());
                }
            }
        }

        max - min
    }

    pub fn text_width(&self, text: String) -> f64 {
        let mut width: f64 = 0.0;

        for character in text.chars() {
            let glyph = self.glyphs.iter().find(|g| g.unicode == character as u32);

            if let Some(glyph) = glyph {
                width += glyph.horizontal_advance_x;
            }
        }

        width - self.left_margin_for_text(text)
    }

    pub fn highest_glyph(&self) -> char {
        let mut highest_glyph = Glyph {
            unicode: self.glyphs[0].unicode,
            horizontal_advance_x: self.glyphs[0].horizontal_advance_x,
            path: String::from(self.glyphs[0].path.clone()),
        };

        let path = Path::new(highest_glyph.path.clone()).unwrap();

        let mut highest_y = path.bounds().unwrap().y1();

        for glyph in &self.glyphs {
            if let Ok(path) = Path::new(glyph.path.clone()) {
                let bounds = path.bounds().unwrap();

                if bounds.y1() < highest_y {
                    highest_y = bounds.y1();
                    highest_glyph.unicode = glyph.unicode;
                    highest_glyph.horizontal_advance_x = glyph.horizontal_advance_x;
                    highest_glyph.path = glyph.path.clone();
                }
            }
        }

        from_u32(highest_glyph.unicode).unwrap()
    }

    fn left_margin_for_text(&self, text: String) -> f64 {
        for character in text.chars() {
            let glyph = self.glyphs.iter().find(|g| g.unicode == character as u32);

            if let Some(glyph) = glyph {
                if let Ok(path) = Path::new(glyph.path.clone()) {
                    let bounds = path.bounds().unwrap();

                    return bounds.x1();
                }
            }
        }

        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARIAL_SVG_FONT: &'static str = include_str!("../../fixtures/arial.svg");

    #[test]
    fn it_loads_font() {
        let data = r#"
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
                <defs>
                    <font id="font" horiz-adv-x="1000">
                        <font-face font-family="font" units-per-em="1000" ascent="800" descent="-200" x-height="500" />

                        <glyph glyph-name="eacute" unicode="&#xe9;" d="M590 -25q-234 0 -376 146.5t-142 408.5q0 260 137.5 407.5t359.5 147.5q227 0 362.5 -145t135.5 -410v-45h-799q12 -184 101.5 -273t220.5 -89q99 0 178.5 50.5t83.5 143.5h195q-22 -164 -148.5 -253t-308.5 -89zM868 633q-23 153 -103 229t-196 76q-113 0 -194 -75.5 t-103 -229.5h596zM826 1491l-221 -281h-148l133 281h236z" />
                        <glyph glyph-name="egrave" unicode="&#xe8;" d="M590 -25q-234 0 -376 146.5t-142 408.5q0 260 137.5 407.5t359.5 147.5q227 0 362.5 -145t135.5 -410v-45h-799q12 -184 101.5 -273t220.5 -89q99 0 178.5 50.5t83.5 143.5h195q-22 -164 -148.5 -253t-308.5 -89zM868 633q-23 153 -103 229t-196 76q-113 0 -194 -75.5t-103 -229.5h596zM684 1210h-148l-221 281h236z" />
                    </font>
                </defs>
            </svg>"#;

        let font = Font::new(data.to_string()).unwrap();

        assert_eq!(font.horizontal_advance_x, 1000.0);
        assert_eq!(font.units_per_em, 1000.0);
        assert_eq!(font.ascent, 800.0);
        assert_eq!(font.descent, -200.0);
        assert_eq!(font.glyphs.len(), 2);

        assert_eq!(font.glyphs[0].unicode, 233);
        assert_eq!(font.glyphs[0].horizontal_advance_x, 1000.0);
        assert_eq!(font.glyphs[0].path, "M590 -25q-234 0 -376 146.5t-142 408.5q0 260 137.5 407.5t359.5 147.5q227 0 362.5 -145t135.5 -410v-45h-799q12 -184 101.5 -273t220.5 -89q99 0 178.5 50.5t83.5 143.5h195q-22 -164 -148.5 -253t-308.5 -89zM868 633q-23 153 -103 229t-196 76q-113 0 -194 -75.5 t-103 -229.5h596zM826 1491l-221 -281h-148l133 281h236z");

        assert_eq!(font.glyphs[1].unicode, 232);
        assert_eq!(font.glyphs[1].horizontal_advance_x, 1000.0);
        assert_eq!(font.glyphs[1].path, "M590 -25q-234 0 -376 146.5t-142 408.5q0 260 137.5 407.5t359.5 147.5q227 0 362.5 -145t135.5 -410v-45h-799q12 -184 101.5 -273t220.5 -89q99 0 178.5 50.5t83.5 143.5h195q-22 -164 -148.5 -253t-308.5 -89zM868 633q-23 153 -103 229t-196 76q-113 0 -194 -75.5t-103 -229.5h596zM684 1210h-148l-221 281h236z");
    }

    #[test]
    fn it_computes_the_correct_arial_font_height() {
        let font = Font::new(ARIAL_SVG_FONT.to_string()).unwrap();

        assert_eq!(font.font_height(), 2270.0);
    }

    #[test]
    fn it_computes_the_arial_font_height_for_a_given_text() {
        let font = Font::new(ARIAL_SVG_FONT.to_string()).unwrap();

        assert_eq!(font.text_height("Hello World".to_string()), 1491.0);
    }

    #[test]
    fn it_computes_the_arial_font_width_for_a_given_text() {
        let font = Font::new(ARIAL_SVG_FONT.to_string()).unwrap();

        assert_eq!(font.text_width("a".to_string()), 1061.0);
    }

    #[test]
    fn it_finds_the_highest_arial_font_glyph() {
        let font = Font::new(ARIAL_SVG_FONT.to_string()).unwrap();

        assert_eq!(font.highest_glyph().to_string(), "g");
    }
}
