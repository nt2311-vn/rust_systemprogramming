use actix_web::{App, HttpResponse, HttpServer, Responder, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(get_index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="POST">
        <input type="text" name="n" />
        <input type="text" name="m" />
        <button type="submit">Compute GCD</button>
    </form>
    "#,
    )
}
