use crate::types::Value;
use crate::error;

pub fn stringify(res: &Value, newline: bool) -> String {
    match res {
        Value::Str(x) => {
            if !newline || x.ends_with("\n") {
                format!("{}", x)
            } else {
                format!("{}\n", x)
            }
        }

        Value::Int(x) => {
            if newline {
                format!("{}\n", x)
            } else {
                format!("{}", x)
            }
        }

        Value::Table(x) => {
            let mut maxes = x.headers.iter().map(|x| stringify(x, false).len() + 1).collect::<Vec<usize>>();
            for i in x.rows.iter() {
                for n in 0..i.len() {
                    if stringify(&i[n], false).len() + 1 > maxes[n] {
                        maxes[n] = stringify(&i[n], false).len() + 1;
                    }
                }
            }

            let mut out = String::new();

            for i in 0..x.headers.len() {
                out.push_str(&error::bold(format!("{:width$}", stringify(&x.headers[i], false), width = maxes[i])));
            }

            out += "\n";

            for row in x.rows.iter() {
                for i in 0..row.len() {
                    out.push_str(&format!("{:width$}", stringify(&row[i], false), width = maxes[i]));
                }

                out += "\n";
            }

            out
        }

        Value::Nil => {
            String::new()
        }

        x => {
            if newline {
                format!("{:?}\n", x)
            } else {
                format!("{:?}", x)
            }
        }
    }
}

pub fn output(res: &Value) {
    print!("{}", stringify(res, true));
}
