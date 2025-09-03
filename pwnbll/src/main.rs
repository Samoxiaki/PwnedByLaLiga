use std::env;
use std::net::SocketAddr;
use tiny_http::{Server, Response};


fn print_error_and_exit(msg: &str) {
	eprintln!("{}", msg);
	eprintln!("Usage: pwnbll <port> <endpoint>");
	std::process::exit(1);
}
fn main() {
    let args: Vec<String> = env::args().collect();
	
    if args.len() != 3 {
		print_error_and_exit("Invalid number of arguments");
	}

	
	let port_num_parsed: Result<u32, _> = args[1].parse();
    if port_num_parsed.is_err() {
		print_error_and_exit("Invalid port number");
	}

	let port: u32 = port_num_parsed.unwrap();
	if port < 1 || port > 65535 {
		print_error_and_exit("Invalid port number (1-65535)");
	}


    let mut endpoint: String = args[2].clone();
    for c in endpoint.chars() {
        if c.is_whitespace() || c == '/' || !c.is_alphanumeric() {
			print_error_and_exit("Invalid endpoint. Only alphanumeric characters allowed and no spaces or slashes");
		}
    }

	endpoint = format!("/{}", endpoint);

    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
    let server = Server::http(addr).unwrap();

    println!("Server running on http://{}{}", addr, endpoint);

    for request in server.incoming_requests() {
		let remote_address = match request.remote_addr() {
			Some(addr) => addr.to_string(),
			None => "Unknown".to_string(),
		};
        
		let path = request.url();
		
		let status_code = if path == endpoint && request.method().as_str() == "GET" {
            200
        } else {
			404
        };

		let data = match status_code {
			200 => "OK",
			_ => "Not Found",
		};

		let response = Response::from_string(data).with_status_code(status_code);

		println!("Request from {} at {} | {} {}", remote_address, path, status_code, data);
		let _ = request.respond(response);
    }
}
