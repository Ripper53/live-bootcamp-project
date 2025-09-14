use pingora::prelude::*;

fn main() {
    println!("starting proxy server");
    let mut server = Server::new(None).expect("proxy server failed to start");
    server.bootstrap();

    let mut redirect = http_proxy_service(&server.configuration, ServerProxy);
    redirect.add_tcp("0.0.0.0:80");
    server.add_service(redirect);

    println!("listening on port 80");
    server.run_forever();
}

struct ServerProxy;

struct ServerProxyContext {
    call: CallType,
}

enum CallType {
    App,
    Auth,
}

#[async_trait::async_trait]
impl ProxyHttp for ServerProxy {
    type CTX = ServerProxyContext;
    fn new_ctx(&self) -> Self::CTX {
        ServerProxyContext {
            call: CallType::App,
        }
    }
    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool> {
        let header = session.req_header_mut();
        let mut parts = header.uri.clone().into_parts();
        if let Some(path) = parts.path_and_query {
            let path = path.as_str();
            const AUTH_PATH: &str = "/auth";
            if let Some(path) = path.strip_prefix(AUTH_PATH) {
                parts.path_and_query = Some(path.parse().unwrap());
                header.set_uri(http::Uri::from_parts(parts).unwrap());
                ctx.call = CallType::Auth;
                return Ok(false);
            }
        }
        Ok(false)
    }
    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let address = match ctx.call {
            CallType::App => "app-service:8000",
            CallType::Auth => "auth-service:3000",
        };
        Ok(Box::new(HttpPeer::new(address, false, "localhost".into())))
    }
}
