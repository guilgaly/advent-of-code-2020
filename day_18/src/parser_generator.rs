use peg::error::ParseError;
use std::fmt::Display;

pub fn eval_flat(expression: &str) -> Result<usize, String> {
    arithmetic::flat(expression).map_err(|e| fmt_err(expression, &e))
}

pub fn eval_reversed(expression: &str) -> Result<usize, String> {
    arithmetic::reversed(expression).map_err(|e| fmt_err(expression, &e))
}

fn fmt_err<T: Display>(expression: &str, error: &ParseError<T>) -> String {
    format!("Cannot evaluate '{}'; {}", expression, error)
}

peg::parser! {
    grammar arithmetic() for str {
        rule number() -> usize = _ n:$(['0'..='9']+) _ { n.parse().unwrap() }

        pub rule flat() -> usize = precedence!{
            x:(@) "+" y:@ { x + y }
            x:(@) "*" y:@ { x * y }
            --
            n:number() { n }
            _ "(" e:flat() ")" _ { e }
        }

        pub rule reversed() -> usize = precedence!{
            x:(@) "*" y:@ { x * y }
            --
            x:(@) "+" y:@ { x + y }
            --
            n:number() { n }
            _ "(" e:reversed() ")" _ { e }
        }

        rule _() = quiet!{[c if c.is_whitespace()]*}
    }
}
