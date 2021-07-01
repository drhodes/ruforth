use crate::types::Data;
use crate::types::Data::{Float, Int};
use crate::types::{Func, Mach, PCode};
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::Rc;
use std::str::FromStr;
// (setq lsp-eldoc-hook nil)

impl Mach {
    pub fn new() -> Mach {
        let mut mach = Mach {
            ds: vec![],
            cStack: vec![],
            words: vec![],
            initCode: String::new(),
            cDict: HashMap::new(),
            rDict: HashMap::new(),
        };
        mach.add_runtime_func("+", rAdd);
        mach.add_runtime_func(".", rDot);
        return mach;
    }

    pub fn add_runtime_func(&mut self, name: &str, f: fn(&mut Mach) -> Option<Data>) {
        let s: String = String::from(name);
        self.rDict.insert(s, PCode::PRFunc(f));
    }
}

pub fn main_loop(mut mach: Mach) {
    loop {
        let (m, mut pcode) = compile(mach);
        mach = m;
        match pcode {
            None => return,
            Some(code) => mach = execute(mach, code),
        }
    }
}

pub fn getWord(mut mach: Mach, prompt: &'static str) -> (Mach, Option<String>) {
    let mut lin = String::new();
    while !(mach.words.len() > 0) {
        if mach.initCode.len() > 0 {
            lin = mach.initCode.clone();
            mach.initCode.clear();
        } else {
            std::io::stdout().write_all(prompt.as_bytes());
            std::io::stdout().flush();
            std::io::stdin().read_line(&mut lin);
        }
        let mut words: Vec<String> = tokenize_words(lin.clone());
        for w in words {
            mach.words.push(w)
        }
    }
    let mut word = &mach.words[0];
    match word.as_str() {
        "bye" => return (mach, None),
        "@ds" => {
            // println!("{:?}", mach.ds);
            let word = mach.words.remove(0);
            return (mach, Some(word));
        }
        _ => {
            let word = mach.words.remove(0);
            return (mach, Some(word));
        }
    }
}

pub fn compile(mut mach: Mach) -> (Mach, Option<Vec<PCode>>) {
    // println!("COMPILE -------------------------------------------------------");
    let mut pcode: Vec<PCode> = vec![];
    let mut prompt = "Forth> ";

    loop {
        match getWord(mach, prompt) {
            (m, None) => {
                mach = m;
                return (mach, None);
            }
            (m, Some(word)) => {
                // println!("compile.getWord -> {:?}", word);
                mach = m;
                if let Some(rAct) = mach.rDict.get(&word) {
                    // println!("compile.rDict[{:?}] -> func ", word);
                    match rAct {
                        PCode::PList(_) => {
                            pcode.push(PCode::PFunc(rRun));
                            pcode.push(PCode::PWord(word));
                        }
                        x => {
                            // println!("pushing {:?}", x);
                            pcode.push(x.clone())
                        }
                    }
                } else {
                    // # Number to be pushed onto ds at runtime
                    // println!("compile.push_to_data_stack <- rpush");
                    pcode.push(PCode::PFunc(rPush));
                    match i64::from_str(&word) {
                        Ok(n) => pcode.push(PCode::PData(Data::Int(n))),
                        Err(_) => {
                            pcode.pop();
                            pcode.push(PCode::PFunc(rRun));
                            pcode.push(PCode::PWord(word));
                        }
                    }
                }
            }
        }
        return (mach, Some(pcode));
    } // end loop
}

pub fn execute(mut mach: Mach, code: Vec<PCode>) -> Mach {
    // println!("EXECUTING  {:?} ------------------", code);
    // println!("Data Stack {:?} -----------", mach.ds);

    let mut p: usize = 0;

    while p < code.len() {
        match &code[p] {
            PCode::PList(_) => todo!(),
            PCode::PFunc(func) => {
                p += 1;
                let (m, newP) = func(mach, &code, p);
                mach = m;
                if let Some(q) = newP {
                    p = q;
                }
            }
            PCode::PRFunc(func) => {
                p += 1;
                let data = func(&mut mach);
                if let Some(Int(newP)) = data {
                    p = newP as usize;
                }
            }
            PCode::PWord(_) => todo!(),
            PCode::PData(data) => {
                // println!("PData {:?}", data);
                p += 1;
            }
        }
    }
    return mach;
}

fn rAdd(m: &mut Mach) -> Option<Data> {
    rBinop(m, &|x, y| x + y)
}

fn rDot(m: &mut Mach) -> Option<Data> {
    match m.ds.pop() {
        Some(PCode::PData(Int(x))) => println!("{:?}", x),
        Some(PCode::PData(Float(x))) => println!("{:?}", x),
        Some(x) => println!("{:?}", x),
        None => println!("empty stack"),
    };
    return None;
}

fn rMul(m: &mut Mach) -> Option<Data> {
    rBinop(m, &|x, y| x * y)
}
fn rSub(m: &mut Mach) -> Option<Data> {
    rBinop(m, &|x, y| x - y)
}
fn rDiv(m: &mut Mach) -> Option<Data> {
    rBinop(m, &|x, y| x / y)
}
fn rEq(m: &mut Mach) -> Option<Data> {
    rBinop(m, &|x, y| if x == y { Int(1) } else { Int(0) })
}
fn rGt(m: &mut Mach) -> Option<Data> {
    rBinop(m, &|x, y| if x > y { Int(1) } else { Int(0) })
}
fn rLt(m: &mut Mach) -> Option<Data> {
    rBinop(m, &|x, y| if x < y { Int(1) } else { Int(0) })
}

fn rBinop(m: &mut Mach, op: &dyn Fn(Data, Data) -> Data) -> Option<Data> {
    let b = m.ds.pop()?;
    let a = m.ds.pop()?;

    match (a, b) {
        (PCode::PData(x), PCode::PData(y)) => {
            m.ds.push(PCode::PData(op(x, y)));
            return None;
        }
        _ => panic!("NOPE!"),
    }
    return None;
}

pub fn tokenize_words(mut s: String) -> Vec<String> {
    let re = Regex::new(r"#.*\n").unwrap();
    s.push_str("\n");
    let result: String = re.replace_all(&mut s, "\n").to_string();
    let mut words = vec![];

    for w in result.split_whitespace() {
        words.push(w.to_owned());
    }
    return words;
}

pub fn rRun(mut mach: Mach, pcode: &Vec<PCode>, p: usize) -> (Mach, Option<usize>) {
    match &pcode[p] {
        PCode::PWord(w) => {
            let f = mach.rDict.get(w).unwrap().clone();
            let m = execute(mach, vec![f]);
            mach = m;
        }
        _ => panic!("OH NO!"),
    }
    return (mach, Some(p + 1));
}

pub fn rPush(mut mach: Mach, pcode: &Vec<PCode>, p: usize) -> (Mach, Option<usize>) {
    let temp = pcode[p].clone();
    mach.ds.push(temp);
    return (mach, Some(p + 1));
}

pub fn cColon(mut mach: Mach) -> Mach {
    if !mach.cStack.len() == 0 {
        panic!(": inside Control stack");
    }
    let (mut mach, label) = getWord(mach, "...");
    mach.cStack.push(("COLON".to_owned(), label.unwrap()));
    return mach;
}
