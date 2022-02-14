/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
use clap::{App, Arg};
use num_cpus;
use std::net::Ipv4Addr;
mod core;

//
// fns:  Find NordVPN Server
//
fn main() {
    let p_start_num = "Start Number";
    let p_end_num = "End Number";
    let p_ip_address = "Ip4 Address";
    let p_country_code = "Country Code";
    let p_thread_count = "Thread Count";
    let p_verbose = "Verbose";

    let matches = App::new("fns")
        .version("1.1")
        .author("Spudmash Media [ - ]")
        .about("Reverse Lookup of NordVPN Server hostname by Ipv4 address\nBuilt with Rust 🦀")
        .arg(
            Arg::new(p_country_code)
                .takes_value(true)
                .short('c')
                .long("country")
                .help("Options: [Al, Ar, Au, At, Be, Ba, Br, Bg, Ca, Cl, Cr, Hr, Cy, Cz, Dk, Ee, Fi, Fr, Ge, De, Gr, Hk, Hu, Is, In, Id, Ie, Il, It, Jp, Lv, Lu, My, Mx, Md, Nl, Nz, Mk, No, Pl, Pt, Ro, Rs, Sg, Sk, Si, Za, Kr, Es, Se, Ch, Tw, Th, Tr, Ua, Uk, Us, Vn]")
                .required(true),
        )
        .arg(
            Arg::new(p_start_num)
                .takes_value(true)
                .short('s')
                .long("start")
                .min_values(1)
                .max_values(999)
                .help("Default: 1"),
        )
        .arg(
            Arg::new(p_end_num)
                .takes_value(true)
                .short('e')
                .long("end")
                .max_values(1000)
                .help("Default: 1000")
                .required(false),
        )
        .arg(
            Arg::new(p_ip_address)
                .takes_value(true)
                .short('i')
                .long("ip")
                .help("Search for VPN Hostname by IP address. E.g. 127.0.0.1")
                .required(true)
                .validator(core::is_valid_ip),
        )
        .arg(
            Arg::new(p_thread_count)
                .takes_value(true)
                .short('t')
                .long("threadcount")
                .help("Thread Count [Default to number of physical CPU cores]")
                .required(false),
        )
        .arg(
            Arg::new(p_verbose)
                .short('v')
                .long("verbose")
                .help("Verbose mode will print out CPU information & suggestions")
                .required(false),
        )
        .get_matches();

    let start_num = matches
        .value_of(&p_start_num)
        .unwrap_or("1")
        .parse::<u32>()
        .unwrap();

    let end_num = matches
        .value_of(&p_end_num)
        .unwrap_or("1000")
        .parse::<u32>()
        .unwrap();

    let match_ip = matches
        .value_of(&p_ip_address)
        .unwrap_or("127.0.0.1")
        .parse::<Ipv4Addr>()
        .unwrap();

    let nat = matches
        .value_of(&p_country_code)
        .unwrap_or("none")
        .parse::<core::enum_country_code::CountryCode>()
        .unwrap();

    let tc = matches
        .value_of(&p_thread_count)
        .unwrap_or(&num_cpus::get().to_string())
        .parse::<usize>()
        .unwrap();

    if matches.is_present(&p_verbose) {
        core::verbose_info(tc.clone());
    }

    std::process::exit(
        match core::entry_point_mt(nat, match_ip, start_num, end_num, tc) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("error: {:?}", err);
                1
            }
        },
    )
}
