pub struct LineChars<'a> {
    pub top_left: &'a str,
    pub top_right: &'a str,
    pub bottom_left: &'a str,
    pub bottom_right: &'a str,
    pub horizontal: &'a str,
    pub vertical: &'a str,
}

pub struct Lines<'a> {
    pub double: LineChars<'a>,
    pub single: LineChars<'a>,
}

pub struct Chars<'a> {
    pub lines: Lines<'a>,
}

pub const CHARS: Chars<'static> = Chars {
    lines: Lines {
        double: LineChars {
            top_left: "\u{2554}",
            top_right: "\u{2557}",
            bottom_left: "\u{255A}",
            bottom_right: "\u{255D}",
            horizontal: "\u{2550}",
            vertical: "\u{2551}",
        },
        single: LineChars {
            top_left: "\u{250C}",
            top_right: "\u{2510}",
            bottom_left: "\u{2514}",
            bottom_right: "\u{2518}",
            horizontal: "\u{2500}",
            vertical: "\u{2502}",
        },
    },
};
