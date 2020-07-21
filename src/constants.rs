pub struct Single<'a> {
    pub top_left: &'a str,
    pub top_right: &'a str,
    pub bottom_left: &'a str,
    pub bottom_right: &'a str,
    pub horizontal: &'a str,
    pub vertical: &'a str,
}

pub struct Lines<'a> {
    pub single: Single<'a>,
}

pub struct Chars<'a> {
    pub lines: Lines<'a>,
}

pub const CHARS: Chars<'static> = Chars {
    lines: Lines {
        single: Single {
            top_left: "\u{2554}",
            top_right: "\u{2557}",
            bottom_left: "\u{255A}",
            bottom_right: "\u{255D}",
            horizontal: "\u{2550}",
            vertical: "\u{2551}",
        },
    },
};
