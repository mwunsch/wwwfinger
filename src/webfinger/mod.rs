extern crate url;
extern crate http;

use std::fmt;
use std::io::{standard_error, IoError, OtherIoError};
use url::Url;
use http::client;
use http::client::response;
use http::method;
use http::headers::request::{UserAgent, Accept};

static USER_AGENT: &'static str = "wwwfinger (Rust) (+https://github.com/mwunsch/wwwfinger)";
static JRD_MEDIA_TYPE: &'static str = "application/jrd+json";
static WEBFIST_HOST: &'static str = "webfist.org";

pub struct WebFinger {
  target: Url,
  maybe_host: Option<StrBuf>
}

pub fn webfist(url: Url) -> WebFinger {
  WebFinger::new(url, Some(WEBFIST_HOST.to_strbuf()))
}

impl WebFinger {
  pub fn new(url: Url, maybe_host: Option<StrBuf>) -> WebFinger {
    WebFinger { target: url, maybe_host: maybe_host }
  }

  pub fn for_resource(url: Url) -> WebFinger {
    WebFinger::new(url, None)
  }

  pub fn is_webfist(&self) -> bool {
    self.maybe_host.clone().map_or(false, |h| h == WEBFIST_HOST.to_strbuf())
  }

  pub fn uri(&self) -> Url {
    let scheme = if self.is_webfist() {
      "http".to_strbuf()
    } else {
      "https".to_strbuf()
    };

    Url {
      scheme: scheme,
      user: None,
      host: self.determined_host(),
      port: None,
      path: "/.well-known/webfinger".to_strbuf(),
      query: vec!(
        ("resource".to_strbuf(), self.target.to_str().to_strbuf())
      ),
      fragment: None
    }
  }

  pub fn request(&self) -> Option<client::RequestWriter> {
    client::RequestWriter::new(method::Get, self.uri()).ok().map(|writer| {
      let mut req = writer;
      req.headers.insert(UserAgent(USER_AGENT.to_strbuf()));
      req.headers.insert(Accept(JRD_MEDIA_TYPE.to_strbuf()));
      req
    })
  }

  pub fn call(&self) -> Result<response::ResponseReader<client::NetworkStream>, IoError> {
    match self.request() {
      Some(req) => { req.read_response().map_err(|(_, err)| err ) }
      None => { Err(standard_error(OtherIoError)) }
    }
  }

  pub fn to_webfist(&self) -> WebFinger {
    webfist(self.target.clone())
  }

  fn determined_host(&self) -> StrBuf {
    self.maybe_host.clone().unwrap_or(
      if self.target.scheme == "acct".to_strbuf() {
        self.target.path.as_slice().split('@').last().unwrap().to_strbuf()
      } else {
        self.target.host.to_strbuf()
      }
    )
  }

}

impl fmt::Show for WebFinger {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.uri())
  }
}
