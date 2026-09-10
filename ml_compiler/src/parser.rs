use crate::enums;
use crate::enums::Token;
use crate::grammer;

pub fn parse(tokens: Vec<Token>) -> Result<enums::Expr, String> {
    let mut context = grammer::ExprContext::with_default_userdata();

    for (index, token) in tokens.iter().cloned().enumerate() {
        if let Err(err) = context.feed(token.clone()) {
            let start = index.saturating_sub(2);
            let end = (index + 3).min(tokens.len());

            let window: Vec<String> = tokens[start..end]
                .iter()
                .enumerate()
                .map(|(i, t)| {
                    let actual_idx = start + i;
                    if actual_idx == index {
                        format!("\x1b[1m{:?}\x1b[0m", t)
                    } else {
                        format!("{:?}", t)
                    }
                })
                .collect();

            let context_str = window.join(", ");

            return Err(format!(
                "parse error at index {} ({}): {}",
                index, context_str, err
            ));
        }
    }

    match context.accept() {
        Ok((ast, _)) => Ok(ast),
        Err(err) => Err(err.to_string()),
    }
}
