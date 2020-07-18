enum Single {
    TopLeft = "\u{2554}",
    TopRight = "\u{2557}",
    BottomLeft = "\u{255A}",
    BottomRight = "\u{255D}",
    Horizontal = "\u{2550}",
    Vertical = "\u{2551}",
}

enum Line {
    Single = Single,
}

pub enum Chars {
    Line = Line,
}
