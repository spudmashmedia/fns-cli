/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
use super::enum_country_code::CountryCode;
use super::enum_match_type::MatchType;
use std::net::Ipv4Addr;

pub struct WorkerRequest {
    pub country_code: CountryCode,
    pub data: Vec<u32>,
}

impl WorkerRequest {
    pub fn new(country_code: CountryCode, data: Vec<u32>) -> WorkerRequest {
        WorkerRequest {
            country_code: country_code,
            data: data,
        }
    }
}

pub struct WorkerResponse {
    pub match_type: MatchType,
    pub host: String,
    pub ip: Ipv4Addr,
}

pub struct SearchFilter {
    pub ip: Ipv4Addr,
}
