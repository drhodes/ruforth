use std::collections::HashMap;
use std::fmt;
// (setq lsp-eldoc-hook nil)

pub enum Ctl {
    Colon(String),
    Begin,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Data {
    Int(i64),
    Float(f64),
}

#[derive(Clone)]
pub enum PCode {
    PList(Vec<PCode>),
    PFunc(Func),
    PRFunc(fn (&mut Mach) -> Option<Data>),
    PWord(String),
    PData(Data),
}

impl fmt::Debug for PCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PCode::PList(pcodes) => {
                let temp = format!("{:?}", pcodes);
                f.write_str(&temp)
            }

            PCode::PFunc(_) => f.write_str("pfunc"), // adelic
            PCode::PRFunc(_) => f.write_str("prfunc"), 
            PCode::PWord(s) => f.write_str(s),
            PCode::PData(data) => {
                let temp = format!("{:?}", data);
                f.write_str(&temp)
            }
        }
    }
}

pub type Func = fn(&mut Mach, &Vec<PCode>, usize) -> Option<usize>;

#[derive(Clone)]
pub struct Mach {
    pub ds: Vec<PCode>,                // the data stack
    pub cStack: Vec<(String, String)>, // The control struct stack
    // cStack: Vec<
    // heap     = [0]*20      // The data heap
    // heapNext =  0          // Next avail slot in heap
    pub words: Vec<String>, // The input stream of tokens
    pub initCode: String,
    pub cDict: HashMap<String, Func>,
    pub rDict: HashMap<String, PCode>,
}
