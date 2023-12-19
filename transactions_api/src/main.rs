use crate::transaction::Transaction;

#[actix::main]
fn main() {
    let t = Transaction::new();
    let server = HttpServer::new();
}
