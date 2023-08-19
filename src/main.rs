use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None )]
struct Args {
    /// get local ip address instead of the remote one
    #[arg(short, long)]
    local: bool,
}

fn main() {

    let args = Args::parse();

    if args.local {
        show_local();
    }
    else {
        show_ip();
    }

}


fn show_ip() {

    // Query the remote service to get the IP
    let resp = reqwest::blocking::get("https://api.ipify.org");

    match resp {

        Ok(r) => {
            let ip = r.text().unwrap();
            println!("{}", &ip);
        },

        Err(_) => {
            println!("network error");
        }
    }
}


fn show_local() {
    use local_ip_address::local_ip;

    let resp = local_ip();

    match resp {

        Ok(ip) => {
            println!("{}", ip);
        },

        Err(_) => {
            println!("failed to get local ip");
        }

    }

}
