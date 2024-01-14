use rustpython_ast::{BoolOp, CmpOp, Expr, ExprAwait, ExprBinOp, ExprBoolOp, ExprCall, ExprCompare, ExprConstant, ExprDict, ExprDictComp, ExprGeneratorExp, ExprIfExp, ExprLambda, ExprListComp, ExprNamedExpr, ExprSet, ExprSetComp, ExprUnaryOp, ExprYield, ExprYieldFrom, Operator, UnaryOp};
use rustpython_ast::bigint::BigInt;
use crate::type_checker::{Environment, KnownTerm, Term};





pub fn infer_type_expr<R>(ast: &Expr<R>, env: &mut Environment) -> Result<Term,String> {
    match ast {
        Expr::Constant(ExprConstant {value, ..}) => Ok(infer_constant(value)),
        Expr::BoolOp(ExprBoolOp{op, mut values, ..}) => {
            match op {
                BoolOp::And => {
                    let mut out = None;
                    while values.len() > 0 {
                        match out {
                            None => {
                                let value1 = infer_type_expr(&values[0], env)?;
                                let value2 = infer_type_expr(&values[1], env)?;

                                if value1.is_unknown() {
                                    out = Some(Term::Unknown);
                                    break;
                                }
                                if value2.is_unknown() {
                                    out = Some(Term::Unknown);
                                    break;
                                }

                                if value1.is_truthy(env) {
                                    if value2.is_truthy(env) {
                                        out = Some(value2);
                                        values = values.split_off(2);
                                    } else {
                                        out = Some(value1);
                                        break;
                                    }
                                } else {
                                    out = Some(value1);
                                    break;
                                }
                            },
                            Some(out_term) => {
                                if out_term.is_truthy(env) {
                                    let value = infer_type_expr(&values[0], env)?;

                                    if value.is_unknown() {
                                        out = Some(Term::Unknown);
                                        break;
                                    }

                                    if value.is_truthy(env) {
                                        out = Some(value);
                                        values = values.split_off(1);

                                    } else {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    //TODO change this
                    out.ok_or("Invalid boolean expression".to_string())
                }
                BoolOp::Or => {
                    let mut out = None;
                    while values.len() > 0 {
                        match &out {
                            None => {
                                let value1 = infer_type_expr(&values[0], env)?;
                                let value2 = infer_type_expr(&values[1], env)?;

                                if value1.is_unknown() {
                                    out = Some(Term::Unknown);
                                    break;
                                }
                                if value2.is_unknown() {
                                    out = Some(Term::Unknown);
                                    break;
                                }

                                if value1.is_truthy(env) {
                                    out = Some(value1);
                                    break;
                                } else if value2.is_truthy(env) {
                                    out = Some(value2);
                                    break;
                                } else {
                                    values = values.split_off(2);
                                }
                            },
                            Some(out_term) => {
                                let value = infer_type_expr(&values[0], env)?;

                                if value.is_unknown() {
                                    out = Some(Term::Unknown);
                                    break;
                                }

                                if out_term.is_truthy(env) {
                                    break;
                                } else if value.is_truthy(env) {
                                    out = Some(value);
                                    break;
                                } else {
                                    values = values.split_off(1);
                                }
                            }
                        }
                    }
                    //TODO change this
                    out.ok_or("Invalid boolean expression".to_string())
                }
            }
        },
        Expr::NamedExpr(ExprNamedExpr {target, value, ..}) => {
            let value = infer_type_expr(value, env)?;

            env.add_variable(target, value.clone());

            Ok(value)
        }
        Expr::BinOp(ExprBinOp {left, op, right, ..}) => {
            let left = infer_type_expr(left, env)?;
            let right = infer_type_expr(right, env)?;

            match op {
                Operator::Add => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Float(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i1 + i2))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(f1 + f2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(i1.to_f64().unwrap() + f2))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Float(Some(f1 + i2.to_f64().unwrap()))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Integer(Some(i1 + BigInt::from(1)))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(1) + i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Integer(Some(i1))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i2))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Float(Some(f1 + 1.0))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(1.0 + f2))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Float(Some(f1))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(f2))),
                                (KnownTerm::String(None), KnownTerm::String(None)) => Term::Known(KnownTerm::String(None)),
                                (KnownTerm::String(Some(s1)), KnownTerm::String(Some(s2))) => Term::Known(KnownTerm::String(Some(s1 + &s2))),
                                (KnownTerm::Bytes(None), KnownTerm::Bytes(None)) => Term::Known(KnownTerm::Bytes(None)),
                                (KnownTerm::Bytes(Some(b1)), KnownTerm::Bytes(Some(b2))) => Term::Known(KnownTerm::Bytes(Some([&b1[..], &b2[..]].concat()))),
                                (KnownTerm::Tuple(None), KnownTerm::Tuple(None)) => Term::Known(KnownTerm::Tuple(None)),
                                (KnownTerm::Tuple(Some(t1)), KnownTerm::Tuple(Some(t2))) => Term::Known(KnownTerm::Tuple(Some([&t1[..], &t2[..]].concat()))),
                                (KnownTerm::Complex { real: None, imag: None }, KnownTerm::Complex { real: None, imag: None }) => Term::Known(KnownTerm::Complex { real: None, imag: None }),
                                (KnownTerm::Complex { real: Some(r1), imag: Some(i1) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) => Term::Known(KnownTerm::Complex { real: Some(r1 + r2), imag: Some(i1 + i2) }),
                                (KnownTerm::Class { .. }, x) => unimplemented!("Add class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("Add class for some type"),
                                _ => return Err(String::from("Invalid type for binary +")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
                Operator::Sub => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Float(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i1 - i2))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(f1 - f2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(i1.to_f64().unwrap() - f2))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Float(Some(f1 - i2.to_f64().unwrap()))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Integer(Some(i1 - BigInt::from(1)))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(1) - i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Integer(Some(i1))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i2))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Float(Some(f1 - 1.0))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(1.0 - f2))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Float(Some(f1))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(f2))),
                                (KnownTerm::Complex { real: None, imag: None }, KnownTerm::Complex { real: None, imag: None }) => Term::Known(KnownTerm::Complex { real: None, imag: None }),
                                (KnownTerm::Complex { real: Some(r1), imag: Some(i1) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) => Term::Known(KnownTerm::Complex { real: Some(r1 - r2), imag: Some(i1 - i2) }),
                                (KnownTerm::Class { .. }, x) => unimplemented!("Minus class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("Minus class for some type"),
                                _ => return Err(String::from("Invalid type for binary -")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
                Operator::Mult => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Float(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i1 * i2))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(f1 * f2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(i1.to_f64().unwrap() * f2))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Float(Some(f1 * i2.to_f64().unwrap()))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Integer(Some(i1))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(0)))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(0)))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Float(Some(f1))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(f2))),
                                (KnownTerm::Float(Some(_)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Float(Some(0.0))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(_))) => Term::Known(KnownTerm::Float(Some(0.0))),
                                (KnownTerm::String(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::String(None)),
                                (KnownTerm::String(Some(s)), KnownTerm::Integer(Some(mut i))) => {
                                    let mut result = String::new();
                                    let one = BigInt::from(1);
                                    while i > BigInt::from(0) {
                                        result += &s;
                                        i -= &one;
                                    }
                                    Term::Known(KnownTerm::String(Some(result)))
                                },
                                (KnownTerm::Bytes(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bytes(None)),
                                (KnownTerm::Bytes(Some(b)), KnownTerm::Integer(Some(mut i))) => {
                                    let mut result = Vec::new();
                                    let one = BigInt::from(1);
                                    while i > BigInt::from(0) {
                                        result.extend_from_slice(&b);
                                        i -= &one;
                                    }
                                    Term::Known(KnownTerm::Bytes(Some(result)))
                                },
                                (KnownTerm::Tuple(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Tuple(None)),
                                (KnownTerm::Tuple(Some(t)), KnownTerm::Integer(Some(mut i))) => {
                                    let mut result = Vec::new();
                                    let one = BigInt::from(1);
                                    while i > BigInt::from(0) {
                                        result.extend_from_slice(&t);
                                        i -= &one;
                                    }
                                    Term::Known(KnownTerm::Tuple(Some(result)))
                                },
                                (KnownTerm::Complex { real: None, imag: None }, KnownTerm::Complex { real: None, imag: None }) => Term::Known(KnownTerm::Complex { real: None, imag: None }),
                                (KnownTerm::Complex { real: Some(r1), imag: Some(i1) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) => Term::Known(KnownTerm::Complex { real: Some(r1 * r2), imag: Some(i1 * i2) }),
                                (KnownTerm::Class { .. }, x) => unimplemented!("Multiply class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("Multiply class for some type"),
                                _ => return Err(String::from("Invalid type for binary *")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
                Operator::MatMult => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Class { .. }, x) => unimplemented!("MatMul class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("MatMul class for some type"),
                                _ => return Err(String::from("Invalid type for binary @ (matmul)")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
                Operator::Div => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Float(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) if i2 != BigInt::from(0) => Term::Known(KnownTerm::Float(Some(i1.to_f64().unwrap() / i2.to_f64().unwrap()))),
                                (KnownTerm::Integer(Some(_)), KnownTerm::Integer(Some(i2))) if i2 == BigInt::from(0) => return Err(String::from("Division by zero")),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) if f2 != 0.0 => Term::Known(KnownTerm::Float(Some(f1 / f2))),
                                (KnownTerm::Float(Some(_)), KnownTerm::Float(Some(f2))) if f2 == 0.0 => return Err(String::from("Division by zero")),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Float(Some(f2))) if f2 != 0.0 => Term::Known(KnownTerm::Float(Some(i1.to_f64().unwrap() / f2))),
                                (KnownTerm::Integer(Some(_)), KnownTerm::Float(Some(f2))) if f2 == 0.0 => return Err(String::from("Division by zero")),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Integer(Some(i2))) if i2 != BigInt::from(0) => Term::Known(KnownTerm::Float(Some(f1 / i2.to_f64().unwrap()))),
                                (KnownTerm::Float(Some(_)), KnownTerm::Integer(Some(i2))) if i2 == BigInt::from(0) => return Err(String::from("Division by zero")),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Float(Some(i1.to_f64().unwrap()))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Float(Some(i2.to_f64().unwrap()))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(false))) => return Err(String::from("Division by zero")),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(_))) => Term::Known(KnownTerm::Float(Some(0.0))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Float(Some(f1))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(1.0 / f2))),
                                (KnownTerm::Float(Some(_)), KnownTerm::Bool(Some(false))) => return Err(String::from("Division by zero")),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(_))) => Term::Known(KnownTerm::Float(Some(0.0))),
                                (KnownTerm::Complex { real: None, imag: None }, KnownTerm::Complex { real: None, imag: None }) => Term::Known(KnownTerm::Complex { real: None, imag: None }),
                                (KnownTerm::Complex { real: Some(r1), imag: Some(i1) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) if r2 != 0.0 && i2 != 0.0 => Term::Known(KnownTerm::Complex { real: Some(r1 / r2), imag: Some(i1 / i2) }),
                                (KnownTerm::Complex { real: Some(_), imag: Some(_) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) if r2 == 0.0 || i2 == 0.0 => return Err(String::from("Division by zero")),
                                (KnownTerm::Class { .. }, x) => unimplemented!("Div class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("Div class for some type"),
                                _ => return Err(String::from("Invalid type for binary /")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
                Operator::Mod => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Float(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) if i2 != BigInt::from(0) => Term::Known(KnownTerm::Float(Some(i1.to_f64().unwrap() % i2.to_f64().unwrap()))),
                                (KnownTerm::Integer(Some(_)), KnownTerm::Integer(Some(i2))) if i2 == BigInt::from(0) => return Err(String::from("Division by zero")),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) if f2 != 0.0 => Term::Known(KnownTerm::Float(Some(f1 % f2))),
                                (KnownTerm::Float(Some(_)), KnownTerm::Float(Some(f2))) if f2 == 0.0 => return Err(String::from("Division by zero")),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Float(Some(f2))) if f2 != 0.0 => Term::Known(KnownTerm::Float(Some(i1.to_f64().unwrap() % f2))),
                                (KnownTerm::Integer(Some(_)), KnownTerm::Float(Some(f2))) if f2 == 0.0 => return Err(String::from("Division by zero")),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Integer(Some(i2))) if i2 != BigInt::from(0) => Term::Known(KnownTerm::Float(Some(f1 % i2.to_f64().unwrap()))),
                                (KnownTerm::Float(Some(_)), KnownTerm::Integer(Some(i2))) if i2 == BigInt::from(0) => return Err(String::from("Division by zero")),
                                (KnownTerm::Integer(Some(_)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Float(Some(0.0))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Float(Some(1.0 % i2.to_f64().unwrap()))),
                                (KnownTerm::Integer(Some(_)), KnownTerm::Bool(Some(false))) => return Err(String::from("Division by zero")),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(_))) => Term::Known(KnownTerm::Float(Some(0.0))),
                                (KnownTerm::Float(Some(_)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Float(Some(0.0))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(1.0 % f2))),
                                (KnownTerm::Float(Some(_)), KnownTerm::Bool(Some(false))) => return Err(String::from("Division by zero")),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(_))) => Term::Known(KnownTerm::Float(Some(0.0))),
                                (KnownTerm::Complex { real: None, imag: None }, KnownTerm::Complex { real: None, imag: None }) => Term::Known(KnownTerm::Complex { real: None, imag: None }),
                                (KnownTerm::Complex { real: Some(r1), imag: Some(i1) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) if r2 != 0.0 && i2 != 0.0 => Term::Known(KnownTerm::Complex { real: Some(r1 % r2), imag: Some(i1 % i2) }),
                                (KnownTerm::Complex { real: Some(_), imag: Some(_) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) if r2 == 0.0 || i2 == 0.0 => return Err(String::from("Division by zero")),
                                (KnownTerm::Class { .. }, x) => unimplemented!("Mod class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("Mod class for some type"),
                                _ => return Err(String::from("Invalid type for binary %")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
                Operator::Pow => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Float(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i1.pow(u32::try_from(i2).unwrap())))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(f1.pow(f2)))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(i1.to_f64().unwrap().pow(f2)))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Float(Some(f1.pow(i2.to_f64().unwrap())))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Integer(Some(i1))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(1)))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(1)))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Float(Some(f1))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some(BigInt::from(1).to_f64().unwrap().pow(f2)))),
                                (KnownTerm::Float(Some(_)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Float(Some(1.0))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(_))) => Term::Known(KnownTerm::Float(Some(0.0))),
                                (KnownTerm::Complex { real: None, imag: None }, KnownTerm::Complex { real: None, imag: None }) => Term::Known(KnownTerm::Complex { real: None, imag: None }),
                                (KnownTerm::Complex { real: Some(r1), imag: Some(i1) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) => Term::Known(KnownTerm::Complex { real: Some(r1.pow(r2)), imag: Some(i1.pow(i2)) }),
                                (KnownTerm::Class { .. }, x) => unimplemented!("Multiply class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("Multiply class for some type"),
                                _ => return Err(String::from("Invalid type for binary *")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
                Operator::LShift => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i1 << i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Integer(Some(i1 << BigInt::from(1)))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(1) << i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Integer(Some(i1 << BigInt::from(0)))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(0) << i2))),
                                (KnownTerm::Class { .. }, x) => unimplemented!("LShift class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("LShift class for some type"),
                                _ => return Err(String::from("Invalid type for binary <<")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
                Operator::RShift => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i1 >> i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Integer(Some(i1 >> BigInt::from(1)))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(1) >> i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Integer(Some(i1 >> BigInt::from(0)))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(0) >> i2))),
                                (KnownTerm::Class { .. }, x) => unimplemented!("RShift class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("RShift class for some type"),
                                _ => return Err(String::from("Invalid type for binary >>")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
                Operator::BitOr => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i1 | i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Integer(Some(i1 | BigInt::from(1)))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(1) | i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Integer(Some(i1 | BigInt::from(0)))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(0) | i2))),
                                (KnownTerm::Class { .. }, x) => unimplemented!("BitOr class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("BitOr class for some type"),
                                _ => return Err(String::from("Invalid type for binary |")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
                Operator::BitXor => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i1 ^ i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Integer(Some(i1 ^ BigInt::from(1)))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(1) ^ i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Integer(Some(i1 ^ BigInt::from(0)))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(0) ^ i2))),
                                (KnownTerm::Class { .. }, x) => unimplemented!("BitXor class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("BitXor class for some type"),
                                _ => return Err(String::from("Invalid type for binary ^")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
                Operator::BitAnd => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i1 & i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Integer(Some(i1 & BigInt::from(1)))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(1) & i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Integer(Some(i1 & BigInt::from(0)))),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(0) & i2))),
                                (KnownTerm::Class { .. }, x) => unimplemented!("BitOr class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("BitOr class for some type"),
                                _ => return Err(String::from("Invalid type for binary |")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
                Operator::FloorDiv => {
                    return if let Term::Known(known1) = left {
                        if let Term::Known(known2) = right {
                            let result = match (known1, known2) {
                                (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Integer(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Bool(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Float(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Float(None)),
                                (KnownTerm::Bool(None), KnownTerm::Bool(None)) => Term::Known(KnownTerm::Integer(None)),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) if i2 != BigInt::from(0) => Term::Known(KnownTerm::Integer(Some(i1 / i2))),
                                (KnownTerm::Integer(Some(_)), KnownTerm::Integer(Some(i2))) if i2 == BigInt::from(0) => return Err(String::from("Division by zero")),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) if f2 != 0.0 => Term::Known(KnownTerm::Float(Some((f1 / f2).floor()))),
                                (KnownTerm::Float(Some(_)), KnownTerm::Float(Some(f2))) if f2 == 0.0 => return Err(String::from("Division by zero")),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Float(Some(f2))) if f2 != 0.0 => Term::Known(KnownTerm::Float(Some((i1.to_f64().unwrap() / f2).floor()))),
                                (KnownTerm::Integer(Some(_)), KnownTerm::Float(Some(f2))) if f2 == 0.0 => return Err(String::from("Division by zero")),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Integer(Some(i2))) if i2 != BigInt::from(0) => Term::Known(KnownTerm::Float(Some((f1 / i2.to_f64().unwrap()).floor()))),
                                (KnownTerm::Float(Some(_)), KnownTerm::Integer(Some(i2))) if i2 == BigInt::from(0) => return Err(String::from("Division by zero")),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Integer(Some(i1))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Integer(Some(i2))),
                                (KnownTerm::Integer(Some(i1)), KnownTerm::Bool(Some(false))) => return Err(String::from("Division by zero")),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(_))) => Term::Known(KnownTerm::Integer(Some(BigInt::from(0)))),
                                (KnownTerm::Float(Some(f1)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Float(Some(f1.floor()))),
                                (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Float(Some((1.0 / f2).floor()))),
                                (KnownTerm::Float(Some(_)), KnownTerm::Bool(Some(false))) => return Err(String::from("Division by zero")),
                                (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(_))) => Term::Known(KnownTerm::Float(Some(0.0))),
                                (KnownTerm::Class { .. }, x) => unimplemented!("FloorDiv class for some type"),
                                (x, KnownTerm::Class { .. }) => unimplemented!("FloorDiv class for some type"),
                                _ => return Err(String::from("Invalid type for binary //")),
                            };
                            Ok(result)
                        } else {
                            Ok(Term::Unknown)
                        }
                    } else {
                        Ok(Term::Unknown)
                    }

                }
            }
        },
        Expr::UnaryOp(ExprUnaryOp { op, operand, ..}) => {
            return match op {
                UnaryOp::Invert => {
                    let value = infer_type_expr(operand, env)?;

                    if let Term::Known(known) = value {
                        let result = match known {
                            KnownTerm::Integer(None) => Term::Known(KnownTerm::Integer(None)),
                            KnownTerm::Integer(Some(i)) => Term::Known(KnownTerm::Integer(Some(!i.clone()))),
                            KnownTerm::Bool(None) => Term::Known(KnownTerm::Integer(None)),
                            KnownTerm::Bool(Some(true)) => Term::Known(KnownTerm::Integer(Some(BigInt::from(-2)))),
                            KnownTerm::Bool(Some(false)) => Term::Known(KnownTerm::Integer(Some(BigInt::from(-1)))),
                            KnownTerm::Class { .. } => unimplemented!("Invert class"),
                            _ => return Err(String::from("Invalid type for unary ~")),
                        };
                        Ok(result)
                    } else {
                        Ok(Term::Unknown)
                    }
                }
                UnaryOp::Not => {
                    let value = infer_type_expr(operand, env)?;

                    if let Term::Known(known) = value {
                        let result = match known {
                            KnownTerm::Integer(None) => Term::Known(KnownTerm::Bool(None)),
                            KnownTerm::Integer(Some(i)) if i != BigInt::from(0) => Term::Known(KnownTerm::Bool(Some(false))),
                            KnownTerm::Integer(Some(i)) if i == BigInt::from(0) => Term::Known(KnownTerm::Bool(Some(true))),
                            KnownTerm::Bool(None) => Term::Known(KnownTerm::Bool(None)),
                            KnownTerm::Bool(b) => Term::Known(KnownTerm::Bool(b.map(|b| !b))),
                            KnownTerm::Class { .. } => unimplemented!("Not class"),
                            KnownTerm::Float(None) => Term::Known(KnownTerm::Bool(None)),
                            KnownTerm::Float(Some(f)) if f != 0.0 => Term::Known(KnownTerm::Bool(Some(false))),
                            KnownTerm::Float(Some(f)) if f == 0.0 => Term::Known(KnownTerm::Bool(Some(true))),
                            KnownTerm::String(None) => Term::Known(KnownTerm::Bool(None)),
                            KnownTerm::String(Some(s)) if s.len() != 0 => Term::Known(KnownTerm::Bool(Some(false))),
                            KnownTerm::String(Some(_)) => Term::Known(KnownTerm::Bool(Some(true))),
                            KnownTerm::Bytes(None) => Term::Known(KnownTerm::Bool(None)),
                            KnownTerm::Bytes(Some(b)) if b.iter().any(|x| x != 0) => Term::Known(KnownTerm::Bool(Some(false))),
                            KnownTerm::Bytes(Some(_)) => Term::Known(KnownTerm::Bool(Some(true))),
                            KnownTerm::Tuple(None) => Term::Known(KnownTerm::Bool(None)),
                            KnownTerm::Tuple(Some(t)) if t.len() != 0 => Term::Known(KnownTerm::Bool(Some(false))),
                            KnownTerm::Tuple(Some(_)) => Term::Known(KnownTerm::Bool(Some(true))),
                            KnownTerm::Complex { .. } => unimplemented!("Not complex"),
                            KnownTerm::Function { .. } => Term::Known(KnownTerm::Bool(Some(false))),// TODO: check this for all cases
                            _ => return Err(String::from("Invalid type for unary not")),
                        };
                        Ok(result)
                    } else {
                        Ok(Term::Unknown)
                    }
                }
                UnaryOp::UAdd => {
                    let value = infer_type_expr(operand, env)?;

                    if let Term::Known(known) = value {
                        let result = match known {
                            KnownTerm::Integer(None) => Term::Known(KnownTerm::Bool(None)),
                            KnownTerm::Integer(Some(i)) => Term::Known(KnownTerm::Integer(Some(i))),
                            KnownTerm::Bool(None) => Term::Known(KnownTerm::Integer(None)),
                            KnownTerm::Bool(Some(true)) => Term::Known(KnownTerm::Integer(Some(BigInt::from(1)))),
                            KnownTerm::Bool(Some(false)) => Term::Known(KnownTerm::Integer(Some(BigInt::from(0)))),
                            KnownTerm::Class { .. } => unimplemented!("Unary + class"),
                            KnownTerm::Float(None) => Term::Known(KnownTerm::Float(None)),
                            KnownTerm::Float(Some(f)) => Term::Known(KnownTerm::Float(Some(f))),
                            KnownTerm::Complex { real, imag } => Term::Known(KnownTerm::Complex { real, imag }),
                            _ => return Err(String::from("Invalid type for unary +")),
                        };
                        Ok(result)
                    } else {
                        Ok(Term::Unknown)
                    }
                }
                UnaryOp::USub => {
                    let value = infer_type_expr(operand, env)?;

                    if let Term::Known(known) = value {
                        let result = match known {
                            KnownTerm::Integer(None) => Term::Known(KnownTerm::Bool(None)),
                            KnownTerm::Integer(Some(i)) => Term::Known(KnownTerm::Integer(Some(i * BigInt::from(-1)))),
                            KnownTerm::Bool(None) => Term::Known(KnownTerm::Integer(None)),
                            KnownTerm::Bool(Some(true)) => Term::Known(KnownTerm::Integer(Some(BigInt::from(-1)))),
                            KnownTerm::Bool(Some(false)) => Term::Known(KnownTerm::Integer(Some(BigInt::from(0)))),
                            KnownTerm::Class { .. } => unimplemented!("Unary - class"),
                            KnownTerm::Float(None) => Term::Known(KnownTerm::Float(None)),
                            KnownTerm::Float(Some(f)) => Term::Known(KnownTerm::Float(Some(f * -1.0))),
                            KnownTerm::Complex { real, imag } => Term::Known(KnownTerm::Complex { real: real.map(|r| r * -1.0), imag }),
                            _ => return Err(String::from("Invalid type for unary -")),
                        };
                        Ok(result)
                    } else {
                        Ok(Term::Unknown)
                    }
                }
            }
        },
        Expr::Lambda(ExprLambda {..}) => unimplemented!("Lambda"),
        Expr::IfExp(ExprIfExp { test, body, orelse, ..}) => {
            let test = infer_type_expr(test, env)?;
            let body = infer_type_expr(body, env)?;
            let orelse = infer_type_expr(orelse, env)?;

            if let Term::Known(known) = test {
                if test.is_truthy(env) {
                    Ok(body)
                } else {
                    Ok(orelse)
                }
            } else {
                Ok(Term::Unknown)
            }
        },
        Expr::Dict(ExprDict {..}) => unimplemented!("Dict"),
        Expr::Set(ExprSet {..}) => unimplemented!("Set"),
        Expr::ListComp(ExprListComp {..}) => unimplemented!("ListComp"),
        Expr::SetComp(ExprSetComp {..}) => unimplemented!("SetComp"),
        Expr::DictComp(ExprDictComp {..}) => unimplemented!("DictComp"),
        Expr::GeneratorExp(ExprGeneratorExp {..}) => unimplemented!("GeneratorExp"),
        Expr::Await(ExprAwait {..}) => unimplemented!("Await"),
        Expr::Yield(ExprYield {..}) => unimplemented!("Yield"),
        Expr::YieldFrom(ExprYieldFrom {..}) => unimplemented!("YieldFrom"),
        Expr::Compare(ExprCompare { left, ops, comparators, ..}) => {
            let left = infer_type_expr(left, env)?;
            let mut comparators = comparators.iter().map(|x| infer_type_expr(x, env)).collect::<Result<Vec<Term>, String>>()?;
            comparators.insert(0, left);
            let mut ops = ops.iter().map(|x| x.clone()).collect::<Vec<CmpOp>>();

            let mut out = Err("Should not be seen".to_string());
            while comparators.len () >= 2 {
                let insert;
                match ops[0] {
                    CmpOp::Eq => {
                        if let Term::Known(a) = &comparators[0] {
                            if let Term::Known(b) = &comparators[1] {
                                let result = match (a,b) {
                                    (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Bool(Some(i1 == i2))),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Bool(Some(i == BigInt::from(1)))),
                                    (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(&BigInt::from(1) == i))),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Bool(Some(i == BigInt::from(0)))),
                                    (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(&BigInt::from(0) == i))),
                                    (KnownTerm::Bool(Some(b1)), KnownTerm::Bool(Some(b2))) => Term::Known(KnownTerm::Bool(Some(b1 == b2))),
                                    (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Bool(Some(f1 == f2))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Bool(Some(f == 1.0))),
                                    (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(1.0 == f))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Bool(Some(f == 0.0))),
                                    (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(0.0 == f))),
                                    (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(i.to_f64().unwrap() == f))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(f == i.to_f64().unwrap()))),
                                    (KnownTerm::String(None), KnownTerm::String(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::String(Some(s1)), KnownTerm::String(Some(s2))) => Term::Known(KnownTerm::Bool(Some(s1 == s2))),
                                    (KnownTerm::Bytes(None), KnownTerm::Bytes(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Bytes(Some(b1)), KnownTerm::Bytes(Some(b2))) => Term::Known(KnownTerm::Bool(Some(b1 == b2))),
                                    (KnownTerm::Tuple(None), KnownTerm::Tuple(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Tuple(Some(t1)), KnownTerm::Tuple(Some(t2))) => Term::Known(KnownTerm::Bool(Some(t1 == t2))),
                                    (KnownTerm::Complex { real: None, imag: None }, KnownTerm::Complex { real: None, imag: None }) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Complex { real: Some(r1), imag: Some(i1) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) => Term::Known(KnownTerm::Bool(Some(r1 == r2 && i1 == i2))),
                                    (KnownTerm::Class { .. }, x) => unimplemented!("Eq class for some type"),
                                    (x, KnownTerm::Class { .. }) => unimplemented!("Eq class for some type"),
                                    _ => return Err(String::from("Invalid type for binary ==")),

                                };
                                if !result.is_truthy(env) {
                                    return Ok(Term::Known(KnownTerm::Bool(Some(false))));
                                } else {
                                    insert = result;
                                }
                            } else {
                                return Ok(Term::Unknown);
                            }
                        } else {
                            return Ok(Term::Unknown);
                        }
                    }
                    CmpOp::NotEq => {
                        if let Term::Known(a) = &comparators[0] {
                            if let Term::Known(b) = &comparators[1] {
                                let result = match (a,b) {
                                    (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Bool(Some(i1 != i2))),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Bool(Some(i != BigInt::from(1)))),
                                    (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(&BigInt::from(1) != i))),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Bool(Some(i != BigInt::from(0)))),
                                    (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(&BigInt::from(0) != i))),
                                    (KnownTerm::Bool(Some(b1)), KnownTerm::Bool(Some(b2))) => Term::Known(KnownTerm::Bool(Some(b1 != b2))),
                                    (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Bool(Some(f1 != f2))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Bool(Some(f != 1.0))),
                                    (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(1.0 != f))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Bool(Some(f != 0.0))),
                                    (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(0.0 != f))),
                                    (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(i.to_f64().unwrap() != f))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(f != i.to_f64().unwrap()))),
                                    (KnownTerm::String(None), KnownTerm::String(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::String(Some(s1)), KnownTerm::String(Some(s2))) => Term::Known(KnownTerm::Bool(Some(s1 != s2))),
                                    (KnownTerm::Bytes(None), KnownTerm::Bytes(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Bytes(Some(b1)), KnownTerm::Bytes(Some(b2))) => Term::Known(KnownTerm::Bool(Some(b1 != b2))),
                                    (KnownTerm::Tuple(None), KnownTerm::Tuple(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Tuple(Some(t1)), KnownTerm::Tuple(Some(t2))) => Term::Known(KnownTerm::Bool(Some(t1 != t2))),
                                    (KnownTerm::Complex { real: None, imag: None }, KnownTerm::Complex { real: None, imag: None }) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Complex { real: Some(r1), imag: Some(i1) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) => Term::Known(KnownTerm::Bool(Some(r1 != r2 && i1 != i2))),
                                    (KnownTerm::Class { .. }, x) => unimplemented!("Neq class for some type"),
                                    (x, KnownTerm::Class { .. }) => unimplemented!("Neq class for some type"),
                                    _ => return Err(String::from("Invalid type for binary !=")),

                                };
                                if !result.is_truthy(env) {
                                    return Ok(Term::Known(KnownTerm::Bool(Some(false))));
                                } else {
                                    insert = result;
                                }
                            } else {
                                return Ok(Term::Unknown);
                            }
                        } else {
                            return Ok(Term::Unknown);
                        }
                    }
                    CmpOp::Lt => {
                        if let Term::Known(a) = &comparators[0] {
                            if let Term::Known(b) = &comparators[1] {
                                let result = match (a, b) {
                                    (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Bool(Some(i1 < i2))),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Bool(Some(i < &BigInt::from(1)))),
                                    (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(&BigInt::from(1) < i))),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Bool(Some(i < &BigInt::from(0)))),
                                    (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(&BigInt::from(0) < i))),
                                    (KnownTerm::Bool(Some(b1)), KnownTerm::Bool(Some(b2))) => Term::Known(KnownTerm::Bool(Some(b1 < b2))),
                                    (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Bool(Some(f1 < f2))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Bool(Some(*f < 1.0))),
                                    (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(1.0 < f))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Bool(Some(*f < 0.0))),
                                    (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(0.0 < f))),
                                    (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(i.to_f64().unwrap() < f))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(*f < i.to_f64().unwrap()))),
                                    (KnownTerm::String(None), KnownTerm::String(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::String(Some(s1)), KnownTerm::String(Some(s2))) => Term::Known(KnownTerm::Bool(Some(s1 < s2))),
                                    (KnownTerm::Bytes(None), KnownTerm::Bytes(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Bytes(Some(b1)), KnownTerm::Bytes(Some(b2))) => Term::Known(KnownTerm::Bool(Some(b1 < b2))),
                                    (KnownTerm::Tuple(None), KnownTerm::Tuple(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Tuple(Some(t1)), KnownTerm::Tuple(Some(t2))) => Term::Known(KnownTerm::Bool(Some(t1 < t2))),
                                    (KnownTerm::Complex { real: None, imag: None }, KnownTerm::Complex { real: None, imag: None }) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Complex { real: Some(r1), imag: Some(i1) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) => Term::Known(KnownTerm::Bool(Some(r1 < r2 && i1 < i2))),
                                    (KnownTerm::Class { .. }, x) => unimplemented!("Lt class for some type"),
                                    (x, KnownTerm::Class { .. }) => unimplemented!("Lt class for some type"),
                                    _ => return Err(String::from("Invalid type for binary <")),
                                };
                                if !result.is_truthy(env) {
                                    return Ok(Term::Known(KnownTerm::Bool(Some(false))));
                                } else {
                                    insert = result;
                                }
                            } else {
                                return Ok(Term::Unknown)
                            }
                        } else {
                            return Ok(Term::Unknown)
                        }
                    }
                    CmpOp::LtE => {
                        if let Term::Known(a) = &comparators[0] {
                            if let Term::Known(b) = &comparators[1] {
                                let result = match (a,b) {
                                    (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Bool(Some(i1 <= i2))),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Bool(Some(i <= &BigInt::from(1)))),
                                    (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(&BigInt::from(1) <= i))),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Bool(Some(i <= &BigInt::from(0)))),
                                    (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(&BigInt::from(0) <= i))),
                                    (KnownTerm::Bool(Some(b1)), KnownTerm::Bool(Some(b2))) => Term::Known(KnownTerm::Bool(Some(b1 <= b2))),
                                    (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Bool(Some(f1 <= f2))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Bool(Some(*f <= 1.0))),
                                    (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(1.0 <= f))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Bool(Some(*f <= 0.0))),
                                    (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(0.0 <= f))),
                                    (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(i.to_f64().unwrap() <= f))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(*f <= i.to_f64().unwrap()))),
                                    (KnownTerm::String(None), KnownTerm::String(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::String(Some(s1)), KnownTerm::String(Some(s2))) => Term::Known(KnownTerm::Bool(Some(s1 <= s2))),
                                    (KnownTerm::Bytes(None), KnownTerm::Bytes(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Bytes(Some(b1)), KnownTerm::Bytes(Some(b2))) => Term::Known(KnownTerm::Bool(Some(b1 <= b2))),
                                    (KnownTerm::Tuple(None), KnownTerm::Tuple(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Tuple(Some(t1)), KnownTerm::Tuple(Some(t2))) => Term::Known(KnownTerm::Bool(Some(t1 <= t2))),
                                    (KnownTerm::Complex { real: None, imag: None }, KnownTerm::Complex { real: None, imag: None }) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Complex { real: Some(r1), imag: Some(i1) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) => Term::Known(KnownTerm::Bool(Some(r1 <= r2 && i1 <= i2))),
                                    (KnownTerm::Class { .. }, x) => unimplemented!("LtEq class for some type"),
                                    (x, KnownTerm::Class { .. }) => unimplemented!("LtEq class for some type"),
                                    _ => return Err(String::from("Invalid type for binary <=")),

                                };
                                if !result.is_truthy(env) {
                                    return Ok(Term::Known(KnownTerm::Bool(Some(false))));
                                } else {
                                    insert = result;
                                }
                            } else {
                                return Ok(Term::Unknown);
                            }
                        } else {
                            return Ok(Term::Unknown);
                        }
                    }
                    CmpOp::Gt => {
                        if let Term::Known(a) = &comparators[0] {
                            if let Term::Known(b) = &comparators[1] {
                                let result = match (a, b) {
                                    (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Bool(Some(i1 > i2))),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Bool(Some(i > &BigInt::from(1)))),
                                    (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(&BigInt::from(1) > i))),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Bool(Some(i > &BigInt::from(0)))),
                                    (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(&BigInt::from(0) > i))),
                                    (KnownTerm::Bool(Some(b1)), KnownTerm::Bool(Some(b2))) => Term::Known(KnownTerm::Bool(Some(b1 > b2))),
                                    (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Bool(Some(f1 > f2))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Bool(Some(*f > 1.0))),
                                    (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(1.0 > f))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Bool(Some(*f > 0.0))),
                                    (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(0.0 > f))),
                                    (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(i.to_f64().unwrap() > f))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(*f > i.to_f64().unwrap()))),
                                    (KnownTerm::String(None), KnownTerm::String(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::String(Some(s1)), KnownTerm::String(Some(s2))) => Term::Known(KnownTerm::Bool(Some(s1 > s2))),
                                    (KnownTerm::Bytes(None), KnownTerm::Bytes(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Bytes(Some(b1)), KnownTerm::Bytes(Some(b2))) => Term::Known(KnownTerm::Bool(Some(b1 > b2))),
                                    (KnownTerm::Tuple(None), KnownTerm::Tuple(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Tuple(Some(t1)), KnownTerm::Tuple(Some(t2))) => Term::Known(KnownTerm::Bool(Some(t1 > t2))),
                                    (KnownTerm::Complex { real: None, imag: None }, KnownTerm::Complex { real: None, imag: None }) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Complex { real: Some(r1), imag: Some(i1) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) => Term::Known(KnownTerm::Bool(Some(r1 > r2 && i1 > i2))),
                                    (KnownTerm::Class { .. }, x) => unimplemented!("Gt class for some type"),
                                    (x, KnownTerm::Class { .. }) => unimplemented!("Gt class for some type"),
                                    _ => return Err(String::from("Invalid type for binary >")),
                                };
                                if !result.is_truthy(env) {
                                    return Ok(Term::Known(KnownTerm::Bool(Some(false))));
                                } else {
                                    insert = result;
                                }
                            } else {
                                return Ok(Term::Unknown)
                            }
                        } else {
                            return Ok(Term::Unknown)
                        };
                    }
                    CmpOp::GtE => {
                        if let Term::Known(a) = &comparators[0] {
                            if let Term::Known(b) = &comparators[1] {
                                let result = match (a, b) {
                                    (KnownTerm::Integer(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Integer(Some(i1)), KnownTerm::Integer(Some(i2))) => Term::Known(KnownTerm::Bool(Some(i1 >= i2))),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Bool(Some(i >= &BigInt::from(1)))),
                                    (KnownTerm::Bool(Some(true)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(&BigInt::from(1) >= i))),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Bool(Some(i >= &BigInt::from(0)))),
                                    (KnownTerm::Bool(Some(false)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(&BigInt::from(0) >= i))),
                                    (KnownTerm::Bool(Some(b1)), KnownTerm::Bool(Some(b2))) => Term::Known(KnownTerm::Bool(Some(b1 >= b2))),
                                    (KnownTerm::Float(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Float(Some(f1)), KnownTerm::Float(Some(f2))) => Term::Known(KnownTerm::Bool(Some(f1 >= f2))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Bool(Some(true))) => Term::Known(KnownTerm::Bool(Some(*f >= 1.0))),
                                    (KnownTerm::Bool(Some(true)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(1.0 >= f))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Bool(Some(false))) => Term::Known(KnownTerm::Bool(Some(*f >= 0.0))),
                                    (KnownTerm::Bool(Some(false)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(0.0 >= f))),
                                    (KnownTerm::Integer(None), KnownTerm::Float(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Float(None), KnownTerm::Integer(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Integer(Some(i)), KnownTerm::Float(Some(f))) => Term::Known(KnownTerm::Bool(Some(i.to_f64().unwrap() >= f))),
                                    (KnownTerm::Float(Some(f)), KnownTerm::Integer(Some(i))) => Term::Known(KnownTerm::Bool(Some(*f >= i.to_f64().unwrap()))),
                                    (KnownTerm::String(None), KnownTerm::String(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::String(Some(s1)), KnownTerm::String(Some(s2))) => Term::Known(KnownTerm::Bool(Some(s1 >= s2))),
                                    (KnownTerm::Bytes(None), KnownTerm::Bytes(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Bytes(Some(b1)), KnownTerm::Bytes(Some(b2))) => Term::Known(KnownTerm::Bool(Some(b1 >= b2))),
                                    (KnownTerm::Tuple(None), KnownTerm::Tuple(None)) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Tuple(Some(t1)), KnownTerm::Tuple(Some(t2))) => Term::Known(KnownTerm::Bool(Some(t1 >= t2))),
                                    (KnownTerm::Complex { real: None, imag: None }, KnownTerm::Complex { real: None, imag: None }) => Term::Known(KnownTerm::Bool(None)),
                                    (KnownTerm::Complex { real: Some(r1), imag: Some(i1) }, KnownTerm::Complex { real: Some(r2), imag: Some(i2) }) => Term::Known(KnownTerm::Bool(Some(r1 >= r2 && i1 >= i2))),
                                    (KnownTerm::Class { .. }, x) => unimplemented!("Gt class for some type"),
                                    (x, KnownTerm::Class { .. }) => unimplemented!("Gt class for some type"),
                                    _ => return Err(String::from("Invalid type for binary >")),
                                };
                                if !result.is_truthy(env) {
                                    return Ok(Term::Known(KnownTerm::Bool(Some(false))));
                                } else {
                                    insert = result;
                                }
                            } else {
                                return Ok(Term::Unknown)
                            }
                        } else {
                            return Ok(Term::Unknown)
                        }
                    }
                    CmpOp::Is => unimplemented!("Is"),
                    CmpOp::IsNot => unimplemented!("IsNot"),
                    CmpOp::In => unimplemented!("In"),
                    CmpOp::NotIn => unimplemented!("NotIn"),
                }
                ops.pop();

                comparators = comparators.split_off(2);
                if comparators.len() == 0 || !insert.is_truthy(env) {
                    out = Ok(insert);
                    break;
                }
                comparators.insert(0, insert);
            }
            return out;
        },
        Expr::Call(ExprCall {func, args, keywords, ..}) => {
            let func = infer_type_expr(func, env)?;
            if let Term::Known(func) = func {
                match func {
                    KnownTerm::Function {..} => unimplemented!("Call function"),
                    _ => return Err(String::from("Cannot call non-function")
                }

            } else {
                Ok(Term::Unknown)
            }
        }
    }

}

fn infer_constant(value: &rustpython_ast::Constant) -> Term {
    match value {
        rustpython_ast::Constant::Int(i) => Term::Known(KnownTerm::Integer(Some(i.clone()))),
        rustpython_ast::Constant::Float(f) => Term::Known(KnownTerm::Float(Some(f.clone()))),
        rustpython_ast::Constant::Str(s) => Term::Known(KnownTerm::String(Some(s.clone()))),
        rustpython_ast::Constant::Bytes(b) => Term::Known(KnownTerm::Bytes(Some(b.clone()))),
        rustpython_ast::Constant::Bool(true) => Term::Known(KnownTerm::Bool(Some(true))),
        rustpython_ast::Constant::Bool(false) => Term::Known(KnownTerm::Bool(Some(false))),
        rustpython_ast::Constant::None => Term::Known(KnownTerm::None),
        rustpython_ast::Constant::Ellipsis => Term::Unknown,
        rustpython_ast::Constant::Tuple(t) => {
            let mut terms = Vec::new();
            for term in t {
                terms.push(infer_constant(term).map(|t| match t {
                    Term::Known(known) => known,
                    _ => unreachable!(),
                }));
            }
            Term::Known(KnownTerm::Tuple(Some(terms)))
        }
        rustpython_ast::Constant::Complex { real, imag } => Term::Known(KnownTerm::Complex { real: Some(*real), imag: Some(*imag) })
    }
}
