use std::error::Error;
use std::{borrow::Cow, fmt::Display};

use nix::unistd::{Group, User};
use serde::{de::Visitor, ser::SerializeTuple, Deserialize, Serialize};

#[derive(Debug)]
pub enum AuthEntrySerdeError {
    UserNotFound(String),
    GroupNotFound(String),
    UserNotInGroup(String, String),
}

impl Display for AuthEntrySerdeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthEntrySerdeError::UserNotFound(user) => write!(f, "No user found for '{user}'"),
            AuthEntrySerdeError::GroupNotFound(group) => write!(f, "No group found for '{group}'"),
            AuthEntrySerdeError::UserNotInGroup(user, group) => {
                write!(f, "User '{user}' is not part of group '{group}'")
            }
        }
    }
}

impl Error for AuthEntrySerdeError {}

/// A struct to deserialize user/group names
/// into [`User`]/[`Group`].
#[derive(Debug)]
pub struct DummyAuthEntry {
    pub user: User,
    pub group: Group,
}

struct AuthEntryVisitor;

impl<'de> Visitor<'de> for AuthEntryVisitor {
    type Value = DummyAuthEntry;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a tuple of user and group")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let user: Cow<str> = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let user = User::from_name(&user)
            .map_err(|_| {
                serde::de::Error::custom(AuthEntrySerdeError::UserNotFound(user.to_string()).to_string())
            })?
            .ok_or_else(|| {
                serde::de::Error::custom(AuthEntrySerdeError::UserNotFound(user.to_string()).to_string())
            })?;

        let group: Cow<str> = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        let group = Group::from_name(&group)
            .map_err(|_| {
                serde::de::Error::custom(AuthEntrySerdeError::GroupNotFound(group.to_string()).to_string())
            })?
            .ok_or_else(|| {
                serde::de::Error::custom(AuthEntrySerdeError::GroupNotFound(group.to_string()).to_string())
            })?;

        if user.gid != group.gid {
            return Err(serde::de::Error::custom(
                AuthEntrySerdeError::UserNotInGroup(user.name, group.name).to_string(),
            ));
        }

        Ok(DummyAuthEntry { user, group })
    }
}

impl<'de> Deserialize<'de> for DummyAuthEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, AuthEntryVisitor)
    }
}

impl Serialize for DummyAuthEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&self.user.name)?;
        tup.serialize_element(&self.group.name)?;
        tup.end()
    }
}

/// Auth entries, which are composed of a [`User`](nix::unistd::User) and its associated [`Group`](nix::unistd::Group).
/// The user should be part of the associated group.
#[derive(Debug, Deserialize, Serialize)]
pub struct DummyAuthConfig {
    pub entries: [DummyAuthEntry; 3],
}

impl Default for DummyAuthConfig {
    fn default() -> Self {
        Self {
            entries: [
                {
                    let user = User::from_name("nobody").unwrap().unwrap();
                    let gid = user.gid;
                    DummyAuthEntry {
                        user,
                        group: Group::from_gid(gid).unwrap().unwrap(),
                    }
                },
                DummyAuthEntry {
                    user: User::from_name("pjdfstest").unwrap().unwrap(),
                    group: Group::from_name("pjdfstest").unwrap().unwrap(),
                },
                DummyAuthEntry {
                    user: User::from_name("tests").unwrap().unwrap(),
                    group: Group::from_name("tests").unwrap().unwrap(),
                },
            ],
        }
    }
}
