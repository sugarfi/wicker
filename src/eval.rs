use crate::commands;
use crate::error::error;
use crate::parser::Node;
use crate::session::{local::LocalSession, Session};
use crate::types::Value;
use std::collections::{HashMap, HashSet};
use std::env;

pub struct Context {
    pub status: usize,
    pub cwd: String,
    pub session: usize,
    pub all_sessions: HashMap<usize, Box<dyn Session>>,
}

impl Context {
    pub fn new() -> Self {
        let mut all: HashMap<usize, Box<dyn Session>> = HashMap::new();
        all.insert(0, Box::new(LocalSession));
        return Self {
            status: 0,
            cwd: env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
            session: 0,
            all_sessions: all,
        };
    }

    pub fn eval(&mut self, node: &Node) -> (usize, Value) {
        match node.clone() {
            Node::Command(c, args) => {
                let mut fargs = Vec::new();
                let mut flags = HashSet::new();
                let mut fvals = HashMap::new();
                // every argument
                let mut all: Vec<Value> = Vec::new();

                for i in args.iter() {
                    match i {
                        Node::Flag(x) => {
                            all.push(Value::Str(x.to_string()));

                            if x.contains('=') {
                                let mut s = x.split('=');
                                let a = s.next().unwrap();
                                let b = s.next().unwrap_or("");
                                fvals.insert(a.to_string(), b.to_string());
                            } else {
                                flags.insert(x.clone());
                            }
                        }

                        _ => {
                            all.push(self.eval(i).1);
                            fargs.push(self.eval(i).1);
                        }
                    }
                }

                if let Some((status, res)) = commands::call(c, &all, &fargs, &flags, &fvals, self) {
                    self.status = status;
                    return (status, res);
                } else {
                    error(format!("command {} failed", c));
                    self.status = 1;
                    return (1, Value::Nil);
                }
            }

            Node::Int(x) => {
                return (0, Value::Int(*x));
            }

            Node::Str(x) => {
                return (0, Value::Str(x.clone()));
            }

            _ => {
                return (0, Value::Nil);
            }
        }
    }
}
