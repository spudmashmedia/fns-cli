/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
pub mod enum_country_code;
use enum_country_code::CountryCode;

mod enum_match_type;
use enum_match_type::MatchType;

mod worker_models;
use worker_models::{SearchFilter, WorkerRequest, WorkerResponse};

use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use std::net::Ipv4Addr;
use std::process::Command;
use std::time::Instant;

use std::sync::mpsc;
use std::thread;

use num_cpus;

// Info: CPU information to help optimize how many threads to spawn with -t option
pub fn verbose_info(requested_thread_count: usize) {
    let available_cpus = &num_cpus::get_physical();
    let available_cpu_threads = &num_cpus::get().to_string();

    println!("\n");
    println!("::CPU Information::");
    println!(" - {} cores available", &available_cpus);
    println!(" - {} threads available", &available_cpu_threads);

    let thread_suggestion = &num_cpus::get() * 2;
    println!(
        "\n💡 Optimize search speed by doubling the thread count or higher. E.g. -t {}\n",
        &thread_suggestion
    );

    println!(
        "🥞 {} Threads requested. Distributing workload...\n",
        &requested_thread_count
    );
}

// Util: build dataframe from start value to end value
fn build_data(start: u32, end: u32) -> Vec<u32> {
    let mut vec = Vec::new();
    for n in start..=end {
        vec.push(n)
    }
    vec
}

// Util: build vpn name
fn get_vpn_string(country: &CountryCode, num: &u32) -> String {
    return format!("{}{}.nordvpn.com", country.to_string(), &num);
}

// Parse ping output string
fn parse_ping_result(ping_data: String) -> Ipv4Addr {
    let data: Vec<&str> = ping_data.split('\n').collect();

    let target = data[0];
    let mut output = "0.0.0.0";

    if target.len() > 0 {
        let start_idx = target.find("(").unwrap() + 1;
        let end_idx = target.find(")").unwrap();
        output = &target[start_idx..end_idx];
    }

    output.parse::<Ipv4Addr>().unwrap()
}

// Util: ping host name, grep ip address
fn ping_by_hostname(hostname: &String) -> Ipv4Addr {
    let mut ping = Command::new("ping");
    ping.arg("-c 1").arg(hostname);

    let result = ping.output().expect("failed to execute process");

    let ping_response = String::from_utf8(result.stdout).unwrap();

    parse_ping_result(ping_response)
}

// worker task:
// - iterate through dataframe and ping each hostname to find a match
// - transmit match back to main thread
fn worker_task(
    pb: ProgressBar,
    tx: mpsc::Sender<WorkerResponse>,
    filter: SearchFilter,
    payload: WorkerRequest,
) {
    let test_empty_ip_address = Ipv4Addr::new(0, 0, 0, 0);

    // scan dataframe sequentially
    for item in payload.data.into_iter() {
        let hostname = get_vpn_string(&payload.country_code, &item);

        pb.set_message(&format!("🔎 [scanning: {}]", &hostname));
        let response = ping_by_hostname(&hostname);

        let response_oct = response.octets();
        let filter_oct = filter.ip.octets();

        // exact match
        if response != test_empty_ip_address && response == filter.ip {
            let result = WorkerResponse {
                match_type: MatchType::Exact,
                host: hostname,
                ip: response,
            };
            tx.send(result).unwrap(); // notify main thread
        } else if response_oct[0] == filter_oct[0]
            && response_oct[1] == filter_oct[1]
            && response_oct[2] == filter_oct[2]
        {
            //partial match
            let result = WorkerResponse {
                match_type: MatchType::Partial,
                host: hostname,
                ip: response,
            };
            tx.send(result).unwrap(); // notify main thread
        }

        pb.inc(1);
    }

    // Signal to main thread of completion
    pb.finish_with_message("✨Done✨");
}

// --Entry Point--
pub fn entry_point_mt(
    country_code: CountryCode,
    match_ip: Ipv4Addr,
    start_num: u32,
    end_num: u32,
    thread_count: usize,
) -> Result<(), ()> {
    let stop_watch = Instant::now();

    // prepare progress bar
    let m = MultiProgress::new();
    let sty = ProgressStyle::default_bar()
        .template("[{elapsed_precise}]  {bar:13} {spinner} {msg}")
        .progress_chars("◼︎□■.");

    // prepare mpsc channel
    let (tx, rx) = mpsc::channel();

    // prepare data
    let vec = build_data(start_num, end_num);

    // chunk dataframe
    let chunk_size = &vec.len() / thread_count; // as even as possible
    let chunk_dataframe: Vec<WorkerRequest> = vec
        .chunks(chunk_size)
        .into_iter()
        .map(|item| WorkerRequest::new(country_code.clone(), item.to_vec()))
        .collect();

    // spawn threads
    for item in chunk_dataframe.into_iter() {
        let pb = m.add(ProgressBar::new(item.data.len() as u64));
        pb.set_style(sty.clone());

        let tx1 = mpsc::Sender::clone(&tx);

        let search_param = SearchFilter {
            ip: match_ip.clone(),
        };

        let _ = thread::spawn(move || {
            worker_task(pb, tx1, search_param, item);
        });
    }

    // join progress bars on all threads
    m.join_and_clear().unwrap();

    // messags received via channel
    println!("\nSearch Results:\n");
    loop {
        match &rx.recv_timeout(std::time::Duration::new(1, 0)) {
            Ok(data) => {
                println!("[{}]\t[ {}, {}]", &data.match_type, &data.host, &data.ip);
            }
            Err(_err) => {
                println!(
                    "\nElapsed Time: {} ({}ms)\n",
                    HumanDuration(stop_watch.elapsed()),
                    stop_watch.elapsed().as_millis()
                );
                break;
            }
        }
    }

    Ok(())
}

//-----------------
// 🧪 UNIT TESTS 🧪
//-----------------
#[cfg(test)]
mod get_vpn_string_tests {
    use super::*;

    #[test]
    fn get_vpn_string_when_au_enum_string_should_return_correct_string() {
        let test_country = CountryCode::Au;
        let test_index = 42;
        let expected_result = "au42.nordvpn.com";

        let actual_result = get_vpn_string(&test_country, &test_index);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn get_vpn_string_when_empty_country_should_return_correct_string() {
        let test_country = CountryCode::Empty;
        let test_index = 42;
        let expected_result = "42.nordvpn.com";

        let actual_result = get_vpn_string(&test_country, &test_index);

        assert_eq!(expected_result, actual_result);
    }
}

#[cfg(test)]
mod build_data_tests {
    use super::*;

    #[test]
    fn build_data_test() {
        let test_start = 1;
        let test_end = 10;
        let expected_result = 10;

        let actual_result = build_data(test_start, test_end);
        assert_eq!(expected_result, actual_result.len());
    }
}

#[cfg(test)]
mod parse_ping_result_tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn parse_ping_result_when_valid_hostname_should_return_ip() {
        let test_ping_result = "PING au548.nordvpn.com (41.42.43.44): 56 data bytes".to_string();

        let expect_ip_string = "41.42.43.44".to_string().parse::<Ipv4Addr>().unwrap();

        let actual_result = parse_ping_result(test_ping_result);

        assert_eq!(expect_ip_string, actual_result);
    }

    #[test]
    fn parse_ping_when_invalid_hostname_empty_should_return_zero_oct() {
        let test_ping_result = "".to_string();

        let expect_ip_string = "0.0.0.0".to_string().parse::<Ipv4Addr>().unwrap();

        let actual_result = parse_ping_result(test_ping_result);

        assert_eq!(expect_ip_string, actual_result);
    }
}
