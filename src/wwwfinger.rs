extern crate getopts;
extern crate url;
extern crate http;

use std::os;
use std::str;
use getopts::{getopts, optflag};
use http::client;
use http::client::response;
use webfinger::WebFinger;

mod webfinger;

static VERSION: &'static str = "alpha";

fn main() {
  let argv = os::args();

  let options = [
    optflag("h","help","Print usage and exit"),
    optflag("v","version","Print version and exit")
  ];

  let matches = match getopts(argv.tail(), options) {
    Ok(m) => { m }
    Err(f) => { fail!(f.to_err_msg()); }
  };

  if matches.opt_present("h") {
    return println!("{}", getopts::usage(argv[0], options));
  }

  if matches.opt_present("v") {
    return println!("{:s} {:s}", argv[0], VERSION);
  }

  let webfinger = match matches.free.as_slice().head() {
    Some(m) => {
      match url::from_str(*m) {
        Ok(u) => { WebFinger::for_resource(u) }
        Err(f) => { fail!("{}", f); }
      }
    }
    None => { return println!("{}", getopts::short_usage(argv[0], options)); }
  };

  match webfinger.call() {
    Ok(response) => { receive(response) }
    Err(err) => { fail!("{}", err) }
  }
}

fn receive(response: response::ResponseReader<client::NetworkStream>) -> () {
  match response.status {
    http::status::Ok => { println!("{}", response_body(response).unwrap()) }
    status @ _ => { fail!("{}", status) }
  }
}

fn response_body(mut r: response::ResponseReader<client::NetworkStream>) -> Option<~str> {
  str::from_utf8_owned(r.read_to_end().unwrap().as_slice().to_owned())
}

