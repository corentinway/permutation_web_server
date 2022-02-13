extern crate permutation_way;

use actix_web::{post, web, App, HttpServer, Responder, Result, middleware, HttpResponse, get};
use permutation_way::PermutationIterator;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct PermutationResponse {
    permutations: Vec<Vec<i32>>,
}
#[derive(Deserialize)]
struct PermutationRequest {
    input: Vec<i32>,
    max: Option<u32>
}


#[get("/")]
async fn index() -> impl Responder {

    let usage = r#"
    usage : <code>curl -i -X POST host:port  -d '{"input": [1, 2, 3, 4, 5, 6]}' -H 'Content-Type: application/json'</code>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(usage)
}

#[post("/")]
async fn permut(request: web::Json<PermutationRequest>) -> Result<impl Responder> {
    println!("Request received");

    let input = &request.input;
    let max = request.max;

    let mut iterator = PermutationIterator::new(input.to_vec());
    if max.is_some() {
        iterator.set_max(max.unwrap());
    }
    

    let mut response = PermutationResponse {
        permutations: Vec::new(),
    };

    iterator.for_each(|permutation| {
        response.permutations.push(permutation);
    });

    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let server = HttpServer::new(|| 
            App::new()
            .wrap(middleware::Logger::default())
            .service(permut)
            .service(index)
        )
        .bind("0.0.0.0:8080")?
        .run();

    println!("Server started, awaiting");

    server.await
}
