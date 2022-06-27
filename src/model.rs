use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::error::LaboriError;

#[derive(FromRow, Serialize)]
pub struct TableCount {
    pub count: i32,
}

#[derive(FromRow, Serialize)]
pub struct Metadata {
    pub xyzhash: u32,
}

#[derive(Debug)]
pub enum Func {
    FINA,
    FINB,
    FINC,
    FLIN,
    PER,
    DUTY,
    PWID,
    TINT,
    FRAT,
    TOT, 
    VPPA,
    VPPB,
    NONE,
}

impl From<&str> for Func {
    fn from(item: &str) -> Self {
        match item {
            "FINA" => Func::FINA,
            "FINB" => Func::FINB,
            "FINC" => Func::FINC,
            "FLIN" => Func::FLIN,
            "PER" => Func::PER,
            "DUTY" => Func::DUTY,
            "PWID" => Func::PWID,
            "TINT" => Func::TINT,
            "FRAT" => Func::FRAT,
            "TOT" => Func::TOT,
            "VPPA" => Func::VPPA,
            "VPPB" => Func::VPPB,
            _ => Func::NONE, 
        }
    }
}

impl From<Func> for &str {
    fn from(item: Func) -> Self {
        match item {
            Func::FINA => "FINA",
            Func::FINB => "FINB",
            Func::FINC => "FINC",
            Func::FLIN => "FLIN", 
            Func::PER => "PER", 
            Func::DUTY => "DUTY",
            Func::PWID => "PWID",
            Func::TINT => "TINT", 
            Func::FRAT => "FRAT", 
            Func::TOT => "TOT",
            Func::VPPA => "VPPA",
            Func::VPPB => "VPPB", 
            Func::NONE => "", 
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Signal {
    Start,
    Stop,
}

#[derive(Debug)]
pub enum State {
    Running,
    Holded,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Trigger { value: String },
}

impl Command {

    fn into_IwatsuCommand(&self) -> Result<String, LaboriError> {

        let mut cmd = "".to_string();      
        match *self {
            Command::Get{ key: x } => {
                match &*x {
                    "Func" => cmd += ":FUNC?",
                    "Interval" => cmd += ":GATE:TIME?",
                    _ => 
                }
            },
            Command::Set{ key: x, value: y} => {
                match &*x {
                    "Func" => cmd += ":FUNC",
                    "Interval" => cmd += ":GATE:TIME",
                }
            },
            Command::Trigger{ value: x } => {
                match &*x {

                }
            },
        }
        "a".to_string()
    }

}


pub enum Key {
    Func(Func),
    Interval(Interval),
}

#[derive(Debug)]
enum Interval {
  TenMicro,
  SubMilli,
  Milli,
  Centi,
  Deci,
  OneSec,
  Deca,
}

