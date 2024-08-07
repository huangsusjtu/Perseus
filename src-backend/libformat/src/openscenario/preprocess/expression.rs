// use syn::parse::Parser;

use crate::openscenario::preprocess::context_tree::{
    Parameter, ParameterDeclarations,
};

// const OPERATION_TOKEN: [&str; 21] = [
//     "-", "round", "floor", "ceil", "sqrt", "sin", "cos", "tan", "asin",
// "acos",     "atan", "sign", "abs", "max", "min", "pow",
//     "*", "/", "%", "+", "-",
// ];
const OPERATION_TOKEN: [&str; 5] = ["*", "/", "%", "+", "-"];
const OPERATION: [fn(a: f64, b: f64) -> f64; 5] = [
    |a, b| a * b,
    |a, b| a / b,
    |a, b| a % b,
    |a, b| a + b,
    |a, b| a - b,
];
const SPEC_OPERATION_TOKEN: [&str; 3] = ["pow", "max", "min"];
const SPEC_OPERATION: [fn(a: f64, b: f64) -> f64; 3] =
    [|a, b| a.powf(b), |a, b| a.max(b), |a, b| a.min(b)];

#[inline]
fn remove_expr_prefix(expr: &str) -> Option<&str> {
    if expr.starts_with("${") && expr.ends_with("}") {
        return Some(&expr[2..expr.len() - 1]);
    }
    if expr.starts_with("$") {
        return Some(&expr[1..]);
    }
    return None;
}

#[inline]
fn is_operation(expr: &str) -> Option<(usize, usize)> {
    for i in 0..OPERATION_TOKEN.len() {
        if expr.starts_with(OPERATION_TOKEN[i]) {
            return Some((OPERATION_TOKEN[i].len(), i));
        }
    }
    return None;
}

#[inline]
fn is_spec_operation(expr: &str) -> Option<(usize, usize, &str, &str)> {
    for i in 0..SPEC_OPERATION_TOKEN.len() {
        if expr.starts_with(SPEC_OPERATION_TOKEN[i]) {
            let first =
                expr.chars().nth(SPEC_OPERATION_TOKEN[i].len()).unwrap();
            if first != ' ' && first != '(' {
                continue;
            }

            let b = SPEC_OPERATION_TOKEN[i].len();
            let (mut left_start, mut left_end, mut right_start, mut right_end) =
                (b, b, b, b);
            let chars = (&expr[b..]).chars();

            let mut j = b;
            let mut level = 0;
            for c in chars {
                if c == '(' {
                    if level == 0 {
                        left_start = j + 1;
                    }
                    level = level + 1;
                } else if c == ')' {
                    if level == 1 {
                        right_end = j;
                    }
                    level = level - 1;
                } else if c == ',' {
                    if level == 1 {
                        left_end = j;
                        right_start = j + 1;
                    }
                }
                j = j + 1;
            }

            let left = &expr[left_start..left_end];
            let right = &expr[right_start..right_end];
            return Some((right_end + 1, i, left, right));
        }
    }
    return None;
}

#[inline]
fn is_number(expr: &str) -> Option<(usize, f64)> {
    let mut i = 0;
    let mut has_dot = false;
    for ch in expr.chars() {
        match ch {
            '0'..='9' => {}
            '.' => {
                if has_dot {
                    return None;
                }
                has_dot = true;
            }
            _ => {
                if i == 0 {
                    return None;
                } else {
                    break;
                }
            }
        }
        i += 1;
    }

    return Some((i, expr[0..i].parse::<f64>().unwrap()));
}

#[inline]
fn is_variable(expr: &str) -> Option<(usize, &str)> {
    if is_number(expr).is_some() || is_operation(expr).is_some() {
        return None;
    }

    let (mut start, mut end) = (0, 0);
    let mut level = 0;
    for c in expr.chars() {
        if c == '$' && end == 0 {
            start = 1;
        } else if c == '{' {
            if end == 1 && start == 1 {
                start = 2;
            }
            level = level + 1;
        } else if c == '}' {
            level = level - 1;
            if level == 0 {
                break;
            }
        } else if c == ' ' {
            break;
        }
        if is_operation(&expr[end..]).is_some() {
            break;
        }
        end = end + 1;
    }
    return Some((end, &expr[start..end]));
}

#[inline]
fn is_sub_expr(expr: &str) -> Option<(usize, &str)> {
    if expr.starts_with("(") {
        let mut end = 0;
        for c in expr.chars() {
            if c == ')' {
                return Some((end + 1, &expr[1..end]));
            }
            end = end + 1;
        }
    }
    return None;
}

// Token 表示，数字、运算符号、括号
#[derive(Debug, Clone)]
pub(crate) enum Token {
    Number(f64),
    Operator(usize),
    #[allow(unused)]
    SpecOperator(usize),
    Variable(String),
}

pub(crate) fn tokenize(
    old_expr: &str, parameters: &ParameterDeclarations,
) -> anyhow::Result<Vec<Token>> {
    let old_expr = old_expr.trim();
    let expr = if let Some(expr) = remove_expr_prefix(old_expr) {
        expr
    } else {
        old_expr
    };

    let mut tokens = Vec::new();

    let mut index = 0;
    while index < expr.len() {
        let tmp_expr = &expr[index..];
        if tmp_expr.starts_with(" ") {
            index = index + 1;
            continue;
        }

        if let Some(r) = is_number(tmp_expr) {
            tokens.push(Token::Number(r.1));
            index += r.0;
        } else if let Some(r) = is_operation(tmp_expr) {
            tokens.push(Token::Operator(r.1));
            index += r.0;
        } else if let Some(r) = is_spec_operation(tmp_expr) {
            let l1 = handler_expression(r.2, parameters)?;
            let l2 = handler_expression(r.3, parameters)?;
            let result = SPEC_OPERATION[r.1](l1.parse()?, l2.parse()?);

            tokens.push(Token::Number(result));
            index += r.0;
        } else if let Some(r) = is_sub_expr(tmp_expr) {
            tokens.push(Token::Number(
                handler_expression(r.1, parameters)?.parse()?,
            ));
            index += r.0;
        } else if let Some(r) = is_variable(tmp_expr) {
            tokens.push(Token::Variable(r.1.to_string()));
            index += r.0;
        } else {
            return Err(anyhow::anyhow!("unknown token"));
        }
    }
    if tokens.is_empty() {
        return Err(anyhow::anyhow!("no tokens"));
    }

    return Ok(tokens);
}

pub(crate) fn calculate(
    tokens: Vec<Token>, parameters: &ParameterDeclarations,
) -> anyhow::Result<String> {
    let mut r = 0.0;
    if tokens.len() == 0 {
        return Err(anyhow::anyhow!("no tokens"));
    } else if tokens.len() > 1 {
        let mut init_r = false;
        let mut last_operation = 0;
        for token in tokens {
            match token {
                Token::Number(n) => {
                    if init_r {
                        r = OPERATION[last_operation](r, n);
                    } else {
                        r = n;
                        init_r = true;
                    }
                }
                Token::Operator(o) => {
                    last_operation = o;
                }
                Token::Variable(v) => {
                    let n = if let Some(p) = parameters.get(&v) {
                        p.clone()
                    } else {
                        return Err(anyhow::anyhow!("no parameters {}", v));
                    };
                    match n {
                        Parameter::Integer(n) => {
                            if init_r {
                                r = OPERATION[last_operation](r, n as f64);
                            } else {
                                r = n as f64;
                                init_r = true;
                            }
                        }
                        Parameter::String(_s) => {}
                        Parameter::Double(n) => {
                            if init_r {
                                r = OPERATION[last_operation](r, n);
                            } else {
                                r = n as f64;
                                init_r = true;
                            }
                        }
                        Parameter::Boolean(_) => {}
                    }
                }
                Token::SpecOperator(_o) => {}
            }
        }

        return Ok(r.to_string());
    } else {
        match tokens[0].clone() {
            Token::Number(n) => {
                return Ok(n.to_string());
            }
            Token::Variable(v) => {
                let n = if let Some(p) = parameters.get(&v) {
                    p.clone()
                } else {
                    return Err(anyhow::anyhow!("no parameters {}", v));
                };
                match n {
                    Parameter::Integer(n) => {
                        return Ok(n.to_string());
                    }
                    Parameter::String(s) => {
                        return Ok(s);
                    }
                    Parameter::Double(n) => {
                        return Ok(n.to_string());
                    }
                    Parameter::Boolean(b) => {
                        return Ok(b.to_string());
                    }
                }
            }
            _ => {
                return Err(anyhow::anyhow!("error Token type"));
            }
        }
    }
}

pub(crate) fn handler_expression(
    expr: &str, parameters: &ParameterDeclarations,
) -> anyhow::Result<String> {
    let t = tokenize(expr, parameters)?;
    calculate(t, parameters)
}

#[cfg(test)]
mod tests {
    use crate::openscenario::preprocess::context_tree::{
        Parameter, ParameterDeclarations,
    };
    use crate::openscenario::preprocess::expression::{
        handler_expression, is_variable, tokenize,
    };

    #[test]
    fn it_works1() {
        let mut parameters = ParameterDeclarations::default();
        parameters.insert("HostVehicle".to_string(), Parameter::Double(1.0));
        parameters.insert("EgoSpeed".to_string(), Parameter::Double(1.0));
        parameters.insert("TargetSpeed".to_string(), Parameter::Double(2.0));
        parameters.insert("ACCTimeGap".to_string(), Parameter::Double(3.0));
        parameters.insert("PedestrianS".to_string(), Parameter::Double(4.0));
        parameters.insert("StartS".to_string(), Parameter::Double(5.0));
        parameters.insert("Speed".to_string(), Parameter::Double(6.0));
        parameters.insert("TrigTime".to_string(), Parameter::Double(7.0));
        parameters.insert("TiltDuration".to_string(), Parameter::Double(1.0));

        let tokens = tokenize("$HostVehicle", &parameters);
        println!("{:#?}", tokens);

        let tokens = tokenize("${$EgoSpeed / 3.6}", &parameters);
        println!("{:#?}", tokens);
        let tokens = tokenize("${$TargetSpeed / 3.6}", &parameters);
        println!("{:#?}", tokens);
        let tokens = tokenize("${0.25 * $EgoSpeed / 3.6}", &parameters);
        println!("{:#?}", tokens);
        let tokens = tokenize("$ACCTimeGap", &parameters);
        println!("{:#?}", tokens);
        let tokens = tokenize("${40.0/3.6}", &parameters);
        println!("{:#?}", tokens);
        let tokens = tokenize("${$PedestrianS + 10}", &parameters);
        println!("{:#?}", tokens);
        let tokens = tokenize("${$StartS + $Speed * $TrigTime}", &parameters);
        println!("{:#?}", tokens);
        let tokens = tokenize(
            "${$StartS + $Speed * ($TrigTime + 1.0 * $TiltDuration / 3.0)}",
            &parameters,
        );
        println!("{:#?}", tokens);
        let tokens = tokenize("${250/3.6}", &parameters);
        println!("{:#?}", tokens);
        let tokens = tokenize("${-1.0/$TrajRadius}", &parameters);
        println!("{:#?}", tokens);
        let tokens = tokenize("${-$Speed}", &parameters);
        println!("{:#?}", tokens);
        let tokens = tokenize("${1.5708 * pow(3.0 / 3.0, 2)}", &parameters);
        println!("{:#?}", tokens);
    }

    #[test]
    fn it_works2() {
        let mut parameters = ParameterDeclarations::default();
        parameters.insert("HostVehicle".to_string(), Parameter::Double(1.0));
        parameters.insert("EgoSpeed".to_string(), Parameter::Double(1.0));
        parameters.insert("TargetSpeed".to_string(), Parameter::Double(2.0));
        parameters.insert("ACCTimeGap".to_string(), Parameter::Double(3.0));
        parameters.insert("PedestrianS".to_string(), Parameter::Double(4.0));
        parameters.insert("StartS".to_string(), Parameter::Double(5.0));
        parameters.insert("Speed".to_string(), Parameter::Double(6.0));
        parameters.insert("TrigTime".to_string(), Parameter::Double(7.0));
        parameters.insert("TiltDuration".to_string(), Parameter::Double(1.0));

        let tokens = handler_expression("$HostVehicle", &parameters);
        println!("{:#?}", tokens);

        let tokens = handler_expression("${$EgoSpeed / 3.6}", &parameters);
        println!("{:#?}", tokens);
        let tokens = handler_expression("${$TargetSpeed / 3.6}", &parameters);
        println!("{:#?}", tokens);
        let tokens =
            handler_expression("${0.25 * $EgoSpeed / 3.6}", &parameters);
        println!("{:#?}", tokens);
        let tokens = handler_expression("$ACCTimeGap", &parameters);
        println!("{:#?}", tokens);
        let tokens = handler_expression("${40.0/3.6}", &parameters);
        println!("{:#?}", tokens);
        let tokens = handler_expression("${$PedestrianS + 10}", &parameters);
        println!("{:#?}", tokens);
        let tokens =
            handler_expression("${$StartS + $Speed * $TrigTime}", &parameters);
        println!("{:#?}", tokens);
        let tokens = handler_expression(
            "${$StartS + $Speed * ($TrigTime + 1.0 * $TiltDuration / 3.0)}",
            &parameters,
        );
        println!("{:#?}", tokens);
        let tokens = handler_expression("${250/3.6}", &parameters);
        println!("{:#?}", tokens);
        let tokens = handler_expression("${-1.0/$TrajRadius}", &parameters);
        println!("{:#?}", tokens);
        let tokens = handler_expression("${-$Speed}", &parameters);
        println!("{:#?}", tokens);
        let tokens =
            handler_expression("${1.5708 * pow(3.0 / 3.0, 2)}", &parameters);
        println!("{:#?}", tokens);
    }

    #[test]
    fn it_works3() {
        let r = is_variable("$StartS + $StartS");
        println!("{:#?}", r);
    }

    #[test]
    fn it_works4() {
        let mut parameters = ParameterDeclarations::default();
        parameters
            .insert("TargetSpeed1KmH".to_string(), Parameter::Double(1.0));
        // let tokens = tokenize("${($TargetSpeed1KmH * 0.8) / 3.6}",
        // &parameters);
        let tokens = tokenize("${1.5708 * pow(1.0 / 3.0, 2)}", &parameters);

        println!("{:#?}", tokens);
    }
}
