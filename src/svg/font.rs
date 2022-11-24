use html_escape::decode_html_entities;
use roxmltree::*;

struct Glyph {
    unicode: i32,
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
            .filter(|n| n.has_tag_name("glyph"))
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
                    unicode: character as i32,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_font() {
        let data = r#"
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
                <defs>
                    <font id="font" horiz-adv-x="1000">
                        <font-face font-family="font" units-per-em="1000" ascent="800" descent="-200" x-height="500" />

                        <glyph glyph-name="afii62881" unicode="&#x651;&#x64b;" horiz-adv-x="433" d="M412 1879" />
                        <glyph glyph-name="afii57506" unicode="&#x67e;" horiz-adv-x="1461" arabic-form="isolated" d="M1461 293z" />
                    </font>
                </defs>
            </svg>"#;

        let font = Font::new(data.to_string()).unwrap();

        assert_eq!(font.horizontal_advance_x, 1000.0);
        assert_eq!(font.units_per_em, 1000.0);
        assert_eq!(font.ascent, 800.0);
        assert_eq!(font.descent, -200.0);
        assert_eq!(font.glyphs.len(), 2);

        assert_eq!(font.glyphs[0].unicode, 1617);
        assert_eq!(font.glyphs[0].horizontal_advance_x, 433.0);
        assert_eq!(font.glyphs[0].path, "M412 1879");

        assert_eq!(font.glyphs[1].unicode, 1662);
        assert_eq!(font.glyphs[1].horizontal_advance_x, 1461.0);
        assert_eq!(font.glyphs[1].path, "M1461 293z");
    }
}
