use std::time::{Duration, Instant};
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::udp::UdpClientConnection;
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RecordType};
use rand::seq::SliceRandom;

fn test_dns_speed(ip: &str, domains: &[&str], num_queries: usize) -> Option<Vec<Duration>> {
    let addr = format!("{}:53", ip);
    let conn = UdpClientConnection::new(addr.parse().unwrap()).ok()?;
    let client = SyncClient::new(conn);

    let mut durations = Vec::with_capacity(num_queries);

    for _ in 0..num_queries {
        let domain = domains.choose(&mut rand::thread_rng()).unwrap();
        let name = Name::from_ascii(domain).unwrap();

        let start = Instant::now();
        let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).ok()?;
        let duration = start.elapsed();

        if !response.answers().is_empty() {
            durations.push(duration);
        }
    }

    Some(durations)
}

fn main() {
    let dns_providers = vec![
        ("Google", "8.8.8.8"),
        ("Cloudflare", "1.1.1.1"),
        ("NextDNS", "45.90.28.0"),
        ("Quad9", "9.9.9.9"),
        ("AdGuard", "94.140.14.14"),
        ("OpenDNS", "208.67.222.222"),
    ];

    let domains = vec![
        "www.duckduckgo.com"
    ];

    let num_queries = 5;

    println!("Testing DNS providers ({} queries each):", num_queries);

    let mut results = Vec::new();

    for (name, ip) in dns_providers {
        println!("\n{}:", name);
        match test_dns_speed(ip, &domains, num_queries) {
            Some(durations) => {
                for (i, duration) in durations.iter().enumerate() {
                    println!("  Query {}: {:.2} ms", i + 1, duration.as_secs_f64() * 1000.0);
                }
                let avg_duration = durations.iter().sum::<Duration>() / durations.len() as u32;
                println!("  Average: {:.2} ms", avg_duration.as_secs_f64() * 1000.0);
                results.push((name.to_string(), avg_duration));
            }
            None => println!("  Failed to test"),
        }
    }

    println!("\nResults sorted by speed:");
    results.sort_by_key(|&(_, duration)| duration);
    for (name, duration) in results {
        println!("{}: {:.2} ms", name, duration.as_secs_f64() * 1000.0);
    }
}
