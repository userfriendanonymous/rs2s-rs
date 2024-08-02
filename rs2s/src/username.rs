use std::{borrow::Cow, fmt::Display};

use serde::{de::DeserializeOwned, Deserialize, Serialize};


// Sorted array.
pub const VALID_CHARS: [char; 65] = [
    '-', '.',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 
    'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 'Z',
    '_',
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 'z',
];

// Sorted array.
pub const ABNORMAL_CHARS: [char; 1] = [
    '.'
];

// Sorted array.
pub const NORMAL_CHARS: [char; 64] = [
    '-',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 
    'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 'Z',
    '_',
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 'z',
];

#[derive(Clone, Copy, Debug)]
pub enum Status {
    Invalid,
    Abnormal,
    Normal,
}

impl Status {
    pub fn is_valid(&self) -> bool {
        !matches!(self, Status::Invalid)
    }

    pub fn is_normal(&self) -> bool {
        matches!(self, Status::Normal)
    }
}

#[derive(Debug, Clone)]
pub struct Value<'a> {
    content: Cow<'a, str>,
    status: Status,
}

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.content().fmt(f)
    }
}

impl<'a> Serialize for Value<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
        self.content().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Value<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> {
        Ok(Self::new(<&'de str>::deserialize(deserializer)?))
    }
}

// impl<'a> DeserializeOwned for Value<'a> {}

impl<'a> Value<'a> {
    pub unsafe fn new_unchecked(content: Cow<'a, str>, status: Status) -> Self {
        Self { content, status }
    }

    pub fn new(content: impl Into<Cow<'a, str>>) -> Self {
        let content = Into::<Cow<'a, str>>::into(content);
        Self {
            status: Self::str_status(&content),
            content
        }
    }

    pub fn str_status(value: &str) -> Status {
        let len = value.len();
        if len < 1 || len > 28 {
            return Status::Invalid;
        }
        if len < 3 || len > 20 {
            return Status::Abnormal;
        }
        for ch in value.chars() {
            if VALID_CHARS.binary_search(&ch).is_ok() {
                if ABNORMAL_CHARS.binary_search(&ch).is_ok() {
                    return Status::Abnormal;
                }
            } else {
                return Status::Invalid;
            }
        }
        Status::Normal
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn is_valid(&self) -> bool {
        self.status.is_valid()
    }

    pub fn is_normal(&self) -> bool {
        self.status.is_normal()
    }
}

impl From<String> for Value<'static> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<'a> Into<String> for Value<'a> {
    fn into(self) -> String {
        self.content.into_owned()
    }
}

impl<'a> AsRef<str> for Value<'a> {
    fn as_ref(&self) -> &str {
        &self.content
    }
}