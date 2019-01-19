use tokesies::filters::Filter;

//Tokenizer.
pub struct SiliconFilter {
    comment_mode : CommentType
}

impl Filter for SiliconFilter {

    fn on_char(&mut self, c: &char) -> (bool, bool) {
        let mut result : (bool, bool);

        if self.comment_mode == CommentType::Off {
            result = match *c {
                ' ' => drop(),
                '\t' => drop(),
                '\n' => drop(),
                '\r' => drop(),
                '\u{C}' => drop(),

                ';' => keep(),
                '_' => part(),
                '{' => keep(),
                '}' => keep(),
                '(' => keep(),
                '[' => keep(),
                ']' => keep(),
                '<' => keep(),
                '>' => keep(),
                '!' => keep(),

                '#' => {
                    self.comment_mode = CommentType::SingleLine;
                    drop()
                },

                _ => keep()
            }
        }
        else if self.comment_mode == CommentType::SingleLine {
            let newline = match *c {
                '\n' => true,
                '\r' => true,

                '#' => {
                    self.comment_mode = CommentType::MultiLine;
                    false
                }
            };

            if newline {
                self.comment_mode = CommentType::Off
            }

            result = drop();
        }
        else if self.comment_mode = CommentType::MultiLine {
            if *c == '#' {
                self.comment_mode = CommentType::TryExit
            }

            result = drop()
        }
        else if self.comment_mode = CommentType::TryExit {
            if *c == '#' {
                self.comment_mode = CommentType::Off
            }
            else {
                self.comment_mode = CommentType::MultiLine
            }

            result = drop()
        }

        return result
    }
}

//Working with Tuples got too tedious.
fn keep() -> (bool, bool) {
    return (true, true)
}

fn drop() -> (bool, bool) {
    return (true, false)
}

fn part() -> (bool, bool) {
    return (false, false)
}

//Comment type
enum CommentType {
    Off,
    SingleLine,
    MultiLine,
    TryExit,
}