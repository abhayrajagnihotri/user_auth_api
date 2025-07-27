// fn main() {
//     println!("Hello, world!");
// }


use axum:: {
    routing::get,
    Router,
};
use std::net::SocketAddr;

asyn fn hello() -> &'static str{
    "Hello from axum"
}

#[tokio::main]
asyn fn main(){
    let app = Router::new().route("/",get(hello));
    let addr = SocketAddr::from(([127,0,0,1],3000));

    println!("server runnning at http://{}", addr);

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}