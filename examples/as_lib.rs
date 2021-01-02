use hostman::{ManagedHostsFile, MatchType};
use hosts_parser::HostsFileLine;

fn main() {
    let host_name = "povilas.linux";
    let ip = "1.2.3.5";

    let mut hosts_file = ManagedHostsFile::must_load();
    let matches = hosts_file.get_multi_match(&[host_name], &MatchType::Exact);
    if !matches.is_empty() {
        hosts_file.remove_host(host_name);
    }

    // TODO(povilas): hostman --> my app name
    let host_line = format!("{} {} # Added by hostman", ip, host_name);
    let line = HostsFileLine::from_string(&host_line).expect("Failed to parse host line");
    hosts_file.add_line(&host_line);
    println!("{}", hosts_file);
}
