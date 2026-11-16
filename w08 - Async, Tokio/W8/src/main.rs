use std::net::TcpListener;
use std::sync::Mutex;
use std::vec;
use axum::Router;
use axum::routing::{get, post};
use tokio::fs::File;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::oneshot;

async fn a2() {
    println!("Hello, a2!");
}

fn not_async() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {});
    rt.block_on(a2());
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    race().await;

    let ab1 = async move {};

    ab1.await;

    tokio::spawn(async move {});

    let f1 = async {};

    let f2 = async {};

    tokio::join!(f1, f2); //Konkurens, de nem párhuzamos!

    println!("Hello, world!");

    let mut1 = Mutex::new(1);
    tokio::spawn(async move {
        mutex_test1(&mut1).await;
    });

    tokio::spawn(async move {
        listener().await;
    });
    connect().await;


    axum_rs().await;
}

async fn axum_rs(){
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/", post(|req : axum::http::Request<axum::body::Body>| async {
            let body = req.into_body();
            let bytes = axum::body::to_bytes(body, 2048).await.unwrap();
            let body_str = String::from_utf8(bytes.to_vec()).unwrap();
            println!("Body: {}", body_str);
            body_str
        }));


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4242").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn race() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async move {
        tx1.send("T1").expect("Cannot send on tx1!");
    });

    tokio::spawn(async move {
        tx2.send("T2").expect("Cannot send on tx2!");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 is received {:?}", val);
        }
        val = rx2 => {
            println!("rx2 received {:?}", val);
        }
    }
}

async fn just_async() {}

async fn mutex_test1(mutex: &std::sync::Mutex<i32>) {
    {
        let mut lock = mutex.lock().unwrap();
        *lock += 1;
    } //A kézi drop sem működik! | Lehet wrapelni structba, +impl + nem async fn
    just_async().await;
}

async fn listener(){
    let ls1 = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    loop {
        let (mut socket, _) = ls1.accept().await.unwrap();

        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}


async fn connect(){
    let s1 = tokio::net::TcpStream::connect("127.0.0.1:3000").await.unwrap();
    let (mut rd, mut wr) = io::split(s1);

    tokio::spawn(async move {
        wr.write_all(b"Msg1").await.unwrap();
        wr.write_all(b"Msg2").await.unwrap();
        wr.write_all(b"Msg3").await.unwrap();

        wr.shutdown().await.unwrap();
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await.unwrap();

        if n == 0 {
            break;
        }

        println!("GOT {:?}", &buf[..n]);
    }

}

async fn read_file(){
    let mut f = File::open("foo.txt").await.unwrap();
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await.unwrap();
    println!("The bytes: {:?}", buffer);
}
