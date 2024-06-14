use std::collections::HashMap;

use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
pub struct IosActivityPushToken {
    pub last_date: String,
    pub push_token: String,
}

pub fn conn_db_ios_activity_push_token() -> Connection {
    let db_path = "./db.db3";
    let conn = Connection::open(db_path).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ios_activity_push_token (
            lastDate TEXT,
            pushToken TEXT PRIMARY KEY
        )",
        [],
    )
    .unwrap();

    conn
}

// token format is a-z0-9, 256 characters
pub fn check_token_format(key: &str) -> bool {
    if key.len() != 256 {
        return false;
    }

    for c in key.chars() {
        if !c.is_ascii_alphanumeric() {
            return false;
        }
    }

    true
}

// GET /ios/activity?token={String}
pub async fn get_ios_activity(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let token = match query.get("token") {
        Some(token) => token,
        None => {
            return HttpResponse::BadRequest().body("Token not found");
        }
    };

    if !check_token_format(token) {
        return HttpResponse::BadRequest().body("Invalid token format");
    }

    let conn = conn_db_ios_activity_push_token();
    let mut ios_token = IosActivityPushToken {
        last_date: String::from(""),
        push_token: String::from(""),
    };

    {
        let mut stmt = conn
            .prepare("SELECT lastDate, pushToken FROM ios_activity_push_token WHERE pushToken = ?")
            .unwrap();
        let mut rows = stmt.query(&[token]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            ios_token = IosActivityPushToken {
                last_date: row.get(0).unwrap(),
                push_token: row.get(1).unwrap(),
            };
        }
    }

    conn.close().unwrap();

    if ios_token.push_token == "" {
        return HttpResponse::NotFound().body("Token not found");
    }

    HttpResponse::Ok().body(serde_json::to_string(&ios_token).unwrap())
}

// POST /ios/activity/<push_token>   if push_token is already int the database, update last_date, otherwise insert
pub async fn post_ios_acitvity(path: web::Path<String>) -> impl Responder {
    let token = path.into_inner();

    if !check_token_format(&token) {
        return HttpResponse::BadRequest().body("Invalid token format");
    }

    let conn = conn_db_ios_activity_push_token();
    let mut ios_token = IosActivityPushToken {
        last_date: String::from(""),
        push_token: String::from(""),
    };

    {
        let mut stmt = conn
            .prepare("SELECT lastDate, pushToken FROM ios_activity_push_token WHERE pushToken = ?")
            .unwrap();
        let mut rows = stmt.query(&[&token]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            ios_token = IosActivityPushToken {
                last_date: row.get(0).unwrap(),
                push_token: row.get(1).unwrap(),
            };
        }

        if ios_token.push_token == "" {
            conn.execute(
            "INSERT INTO ios_activity_push_token (lastDate, pushToken) VALUES (datetime('now'), ?)",
            &[&token],
        )
        .unwrap();
        } else {
            conn.execute(
                "UPDATE ios_activity_push_token SET lastDate = datetime('now') WHERE pushToken = ?",
                &[&token],
            )
            .unwrap();

            let mut stmt2 = conn
                .prepare(
                    "SELECT lastDate, pushToken FROM ios_activity_push_token WHERE pushToken = ?",
                )
                .unwrap();
            let mut rows2 = stmt2.query(&[&token]).unwrap();

            while let Some(row) = rows2.next().unwrap() {
                ios_token = IosActivityPushToken {
                    last_date: row.get(0).unwrap(),
                    push_token: row.get(1).unwrap(),
                };
            }
        }
    }

    conn.close().unwrap();

    HttpResponse::Ok().body(serde_json::to_string(&ios_token).unwrap())
}
