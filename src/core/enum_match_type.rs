/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
use std::fmt;
use std::fmt::{Display, Formatter};

pub enum MatchType {
    Exact,
    Partial,
}

impl Display for MatchType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            MatchType::Exact => write!(f, "{}", "  Exact  "),
            MatchType::Partial => write!(f, "{}", " Partial "),
        }
    }
}
