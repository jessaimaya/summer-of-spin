use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;
use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;
use spin_sdk::sqlite::{Connection, Value};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize)]
struct Game {
    gameId: String,
    message: String,
    grid: String,
    currentRow: i64,
    solved: i64,
}
/// A simple Spin HTTP component.
#[http_component]
fn handle_challenge3(req: Request) -> anyhow::Result<impl IntoResponse> {
    let connection = Connection::open_default()?;
    /*let execute_params = [
        Value::Text("Try out Spin SQLite".to_owned()),
        Value::Text("Friday".to_owned()),
    ];
    connection.execute(
        "INSERT INTO todos (description, due) VALUES (?, ?)",
        execute_params.as_slice(),
    )?;

    let rowset = connection.execute("SELECT id, description, due FROM todos", &[])?;
    println!("rowset: {:?}", rowset);
    let todos: Vec<_> = rowset
        .rows()
        .map(|row| ToDo {
            id: row.get::<u32>("id").unwrap(),
            description: row.get::<&str>("description").unwrap().to_owned(),
            due: row.get::<&str>("due").unwrap().to_owned(),
        })
        .collect();
    let body = serde_json::to_vec(&todos)?;*/

    match req.method() {
        Method::Post => {
            if req.path().contains("start") {
                let gameId = Uuid::new_v4().to_string();
                let message = String::from("The game has started, start guessing the word");
                let grid = String::from("grid");
                let currentRow = 0;
                let solved = 0;

                let word = pick_random_word("list.txt")?;

                let execute_params = [
                    Value::Text(gameId.clone()),
                    Value::Text(message.clone()),
                    Value::Text(grid.clone()),
                    Value::Integer(currentRow),
                    Value::Integer(solved),
                    Value::Text(word.clone()),
                ];

                let game = Game {
                    gameId,
                    message,
                    grid,
                    currentRow,
                    solved,
                };

                connection.execute(
                    "INSERT INTO game (gameId, message, grid, currentRow, solved, word) VALUES (?, ? ,? ,? ,?, ?)",
                    execute_params.as_slice()
                )?;

                let body = serde_json::to_vec(&game)?;

                return Ok(Response::builder()
                    .status(200)
                    .header("content-type", "text/plain")
                    .body(body)
                    .build());
            } else {
                return Ok(Response::builder().status(405).build());
            }
        }
        Method::Get => {
            if req.path().contains("guess") {
                return Ok(Response::builder()
                    .status(200)
                    .header("content-type", "text/plain")
                    .body("")
                    .build());
            } else {
                return Ok(Response::builder().status(405).build());
            }
        }
        _ => {
            return Ok(Response::builder().status(405).build());
        }
    }
}

fn pick_random_word<P: AsRef<Path>>(filename: P) -> io::Result<String> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let words: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut rng = thread_rng();
    if let Some(word) = words.choose(&mut rng) {
        Ok(word.clone())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "No words found"))
    }
}
