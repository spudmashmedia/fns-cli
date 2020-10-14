/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use std::net::Ipv4Addr;
use std::process::Command;

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn chunky_test(count: i32, chunk_size: usize) {
    let mut vec = Vec::new();

    for n in 1..=count {
        vec.push(n)
    }

    let mut new_vec = vec
        .chunks(chunk_size)
        .into_iter()
        .map(|item| ("au", item))
        .into_iter();

    loop {
        let data_iter = new_vec.next();
        match data_iter {
            Some(shit) => {
                println!("{:?}", shit);
            }
            _ => {
                println!("finished");
                break;
            }
        }
    }
}

fn get_vpn_string(country: String, num: u64) -> String {
    return format!("{}{}.nordvpn.com", country, num);
}

fn z_ping_by_hostname(hostname: &String) -> Ipv4Addr {
    let mut ping = Command::new("ping");
    ping.arg("-c 1").arg(hostname);

    let result = ping.output().expect("failed to execute process");

    // println!("got this from ping:\n{:?}", String::from_utf8(result.stdout));

    let atoms = String::from_utf8(result.stdout).unwrap();
    let data: Vec<&str> = atoms.split('\n').collect();

    // println!("got these atoms: {:?}", data[0]);
    let target = data[0];
    // println!("TEST: {:?}", &target);
    let mut output = "0.0.0.0";

    if target.len() > 0 {
        let start_idx = target.find("(").unwrap() + 1;
        let end_idx = target.find(")").unwrap();
        output = &target[start_idx..end_idx];
    }

    output.parse::<Ipv4Addr>().unwrap()
}

fn pinger(hostname: &String) {
    let mut ping = Command::new("ping");
    ping.arg("-c 1").arg(hostname);

    let result = ping.output().expect("failed to execute process");

    // println!("got this from ping:\n{:?}", String::from_utf8(result.stdout));

    let atoms = String::from_utf8(result.stdout).unwrap();
    let data: Vec<&str> = atoms.split('\n').collect();

    // println!("got these atoms: {:?}", data[0]);
    let target = data[0];
    // println!("TEST: {:?}", &target);
    let mut output = "0.0.0.0";

    if target.len() > 0 {
        let start_idx = target.find("(").unwrap() + 1;
        let end_idx = target.find(")").unwrap();
        output = &target[start_idx..end_idx];
    }

    println!("pinger: {}", &output);
}

//
// fns:  Find NordVPN Server
// usage:
//  -s start value  [default: 1]
//  -e end value    [default: 1000]
//  -i ipv4 address [mandatory]
//  -o output file  [optional]
//  -t threads      [default: 2]
//
fn main() {
    let p_start_num = "START_NUM";
    let p_end_num = "END_NUM";
    let p_ip_address = "IP_ADDRESS";
    let p_output_file_path = "OUT_FILE_PATH";

    let matches = App::new("fns")
        .version("1.0")
        .author("Spudmash Media")
        .about("Reverse Lookup of NordVPN Server by ipv4 address")
        .arg(
            Arg::with_name("Start Value")
                .short("s")
                .long("start")
                .value_name(&p_start_num)
                .required(false),
        )
        .arg(
            Arg::with_name("End Value")
                .short("e")
                .long("end")
                .value_name(&p_end_num)
                .required(false),
        )
        .arg(
            Arg::with_name("Ipv4 Address")
                .short("i")
                .long("ip")
                .value_name(&p_ip_address)
                .required(true),
        )
        .arg(
            Arg::with_name("Output File Path")
                .short("o")
                .long("outfile")
                .value_name(&p_output_file_path)
                .required(false),
        )
        .get_matches();

    let mut vec = Vec::new();

    let start_num = 1;
    let end_num = 1000;

    for n in start_num..=end_num {
        vec.push(n)
    }

    let pb = ProgressBar::new(end_num);
    pb.set_style(ProgressStyle::default_bar().progress_chars("#>-"));
    pb.set_position(0);
    pb.reset_eta();

    // let match_ip = "144.48.36.11".parse::<Ipv4Addr>().unwrap();
    let match_ip = matches
        .value_of(&p_ip_address)
        .unwrap_or("0.0.0.0")
        .parse::<Ipv4Addr>()
        .unwrap();

    let null_ip = "0.0.0.0".parse::<Ipv4Addr>().unwrap();

    let mut itr = vec.into_iter();
    let mut exact_match_output = Vec::new();
    let mut partial_match_output = Vec::new();

    loop {
        let data = itr.next();
        pb.inc(1);
        match data {
            Some(index) => {
                let country_code = "au".to_owned();
                let vpn_name = get_vpn_string(country_code, index);
                let result = z_ping_by_hostname(&vpn_name);

                let result_oct = result.octets();
                let match_oct = match_ip.octets();

                if result != null_ip && result == match_ip {
                    exact_match_output.push((vpn_name, result));
                } else if result_oct[0] == match_oct[0]
                    && result_oct[1] == match_oct[1]
                    && result_oct[2] == match_oct[2]
                {
                    partial_match_output.push((vpn_name, result));
                }
            }
            _ => {
                pb.finish_with_message("processing completed\n");
                break;
            }
        }
    }

    if !exact_match_output.is_empty() {
        println!("\nExact match found: {:?}\n", exact_match_output);
    } else if !partial_match_output.is_empty() {
        println!("\nPartial match found: {:?}\n", partial_match_output);
    } else {
        println!("No Matches\n");
    }
}
