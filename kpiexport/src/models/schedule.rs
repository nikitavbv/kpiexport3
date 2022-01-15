use std::fmt::Debug;
use serde::{Serializer, Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSchedule {
    pub entries: Vec<GroupScheduleEntry>,
    #[serde(skip_serializing)]
    pub source: Option<GroupScheduleSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupScheduleEntry {
    subject_id: Option<SubjectId>,

    pub week: ScheduleWeek,
    pub day: ScheduleDay,
    pub index: u8, // first lesson is 0
    pub names: Vec<String>,
    pub lecturers: Vec<String>,
    pub locations: Vec<String>
}

#[derive(Clone, Debug)]
pub enum ScheduleDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday, // we never have lessons on Sunday, but it makes sense to keep it here
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectId(u32);

impl GroupScheduleEntry {

    pub fn new(week: ScheduleWeek,
               day: ScheduleDay,
               index: u8,
               names: Vec<String>,
               lecturers: Vec<String>,
               locations: Vec<String>) -> Self {
        Self {
            subject_id: None,

            week,
            day,
            index,
            names,
            lecturers,
            locations,
        }
    }

    pub fn with_locations(&self, locations: Vec<String>) -> Self {
        Self {
            locations,
            ..self.clone()
        }
    }

    pub fn locations(&self) -> &Vec<String> {
        &self.locations
    }
}

impl ScheduleDay {

    pub fn next(&self) -> Self {
        match &self {
            Self::Monday => Self::Tuesday,
            Self::Tuesday => Self::Wednesday,
            Self::Wednesday => Self::Thursday,
            Self::Thursday => Self::Friday,
            Self::Friday => Self::Saturday,
            Self::Saturday => Self::Sunday,
            Self::Sunday => Self::Monday,
        }
    }

    pub fn from_api_index(index: u8) -> Self {
        match index {
            1 => Self::Monday,
            2 => Self::Tuesday,
            3 => Self::Wednesday,
            4 => Self::Thursday,
            5 => Self::Friday,
            6 => Self::Saturday,
            7 => Self::Sunday,
            other => Self::from_api_index(other - 7)
        }
    }

    pub fn to_index(&self) -> u8 {
        match &self {
            Self::Monday => 0,
            Self::Tuesday => 1,
            Self::Wednesday => 2,
            Self::Thursday => 3,
            Self::Friday => 4,
            Self::Saturday => 5,
            Self::Sunday => 6
        }
    }

    pub fn from_index(index: u8) -> Self {
        Self::from_api_index(index + 1)
    }
}

impl Serialize for ScheduleDay {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_i16(self.to_index() as i16)
    }
}

impl <'de> Deserialize<'de> for ScheduleDay {
    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        Ok(Self::from_index(i16::deserialize(deserializer)? as u8))
    }
}

#[derive(Clone, Debug)]
pub enum ScheduleWeek {
    First,
    Second,
}

impl ScheduleWeek {

    pub fn from_api_index(index: u8) -> Self {
        match index {
            1 => Self::First,
            2 => Self::Second,
            other => Self::from_api_index(other - 2)
        }
    }

    pub fn to_index(&self) -> u8 {
        match &self {
            Self::First => 0,
            Self::Second => 1
        }
    }

    pub fn from_index(index: u8) -> Self {
        Self::from_api_index(index + 1)
    }
}

impl Serialize for ScheduleWeek {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_i16(self.to_index() as i16)
    }
}

impl <'de> Deserialize<'de> for ScheduleWeek {
    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        Ok(Self::from_index(i16::deserialize(deserializer)? as u8))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GroupScheduleSource {
    Parser,
    API,
}

impl GroupScheduleSource {

    pub fn to_string(&self) -> String {
        match &self {
            Self::Parser => "parser".to_string(),
            Self::API => "api".to_string()
        }
    }

    pub fn from_string(name: &str) -> Option<Self> {
        match name {
            "parser" => Some(Self::Parser),
            "api" => Some(Self::API),
            _ => None
        }
    }
}