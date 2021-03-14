use colored::Color as Orig;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color(Orig);

impl From<Orig> for Color {
    fn from(o: Orig) -> Self {
        Self(o)
    }
}

impl Into<Orig> for Color {
    fn into(self) -> Orig {
        self.0
    }
}

impl std::str::FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') {
            if s.len() != 7 {
                return Err(String::from("Invalid color."));
            }

            let red = match s.get(1..3) {
                Some(s) => s,
                None => return Err(String::from("Failed to parse hex.")),
            };
            let red = match u8::from_str_radix(red, 16) {
                Ok(i) => i,
                Err(e) => return Err(format!("Failed to parse hex.: {}", e)),
            };

            let green = match s.get(3..5) {
                Some(s) => s,
                None => return Err(String::from("Failed to parse hex.")),
            };
            let green = match u8::from_str_radix(green, 16) {
                Ok(i) => i,
                Err(e) => return Err(format!("Failed to parse hex.: {}", e)),
            };

            let blue = match s.get(5..7) {
                Some(s) => s,
                None => return Err(String::from("Failed to parse hex.")),
            };
            let blue = match u8::from_str_radix(blue, 16) {
                Ok(i) => i,
                Err(e) => return Err(format!("Failed to parse hex.: {}", e)),
            };

            return Ok(Self(Orig::TrueColor {
                r: red,
                g: green,
                b: blue,
            }));
        }

        Ok(Self(
            Orig::from_str(s).map_err(|_| String::from("Invalid color."))?,
        ))
    }
}

impl Color {
    pub fn parse_until<T: Iterator<Item = char>>(iter: &mut T, ch: char) -> Result<Self, String> {
        use std::str::FromStr;

        let s: String = iter.take_while(|c| *c != ch).collect();
        Self::from_str(&s)
    }
}
