#![allow(unused, dead_code)]

use num::integer::gcd;

pub struct Selector{
    selector: String,
    declarations: Vec<Declaration>,
}

impl Into<String> for Selector {
    fn into(self) -> String {
        let mut decs = "".to_string();
        for d in self.declarations {
            decs.push_str(d.to_string().as_str())
        }
        format!("{}{{{}}}", self.selector, decs)
    }
}

pub struct Declaration {
    property: String,
    value: String,
}

impl Declaration {
    fn to_string(&self) -> String {
        format!("{}:{};", self.property, self.value)
    }
}

struct ScreenRatio(usize, usize);

impl Into<f32> for ScreenRatio {
    fn into(self) -> f32 {
        self.0 as f32 / self.1 as f32
    }
}
fn calc_coprimes(max: usize) -> Vec<ScreenRatio> {
    let mut rv = vec![];
    for x in 1..=max {
        for y in 1..=max {
            if gcd(x, y) == 1 {
                rv.push(ScreenRatio(x, y));
            }
        }
    }
    rv
}

fn main() {
    println!("Hello, world!");
}
