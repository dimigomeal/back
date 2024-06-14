use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use chrono::Timelike;
use rusqlite::Connection;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

// Struct representing iOS activity push token
#[derive(Serialize)]
pub struct IosActivityPushToken {
    pub last_date: String,
    pub push_token: String,
}

// Struct representing a meal
#[derive(Serialize)]
pub struct Meal {
    pub idx: i32,
    pub id: i32,
    pub date: String,
    pub breakfast: String,
    pub lunch: String,
    pub dinner: String,
}

// Establishes a connection to the iOS activity push token database
pub fn conn_db_ios_activity_push_token() -> Connection {
    let db_path = "./db.db3";
    let conn = Connection::open(db_path).unwrap();

    // Create the table if it doesn't exist
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

// Checks if the token format is valid
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

// Handler for GET /ios/activity?token={String}
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

// Handler for POST /ios/activity/<push_token>
// If push_token is already in the database, update last_date, otherwise insert
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
        } else {
            conn.execute(
                "UPDATE ios_activity_push_token SET lastDate = datetime('now') WHERE pushToken = ?",
                &[&token],
            )
            .unwrap();
        }
    }

    conn.close().unwrap();

    HttpResponse::Ok().body(serde_json::to_string(&ios_token).unwrap())
}

/*
    curl -v \
    --header "authorization: bearer ${AUTHENTICATION_TOKEN}" \
    --header "apns-topic: kr.isamin.dimigomeal.push-type.liveactivity" \
    --header "apns-push-type: liveactivity" \
    --header "apns-priority: 10" \
    --header "apns-expiration: 0" \
    --data '{"aps":{"event":"update","content-state":{"type":"<breakfast | lunch | dinner>","meal":"<MEALDATA>","date":"<current YYYY-MM-DD>"},"timestamp":'$(date +%s)'}}' \
    --http2  https://api.development.push.apple.com:443/3/device/${PUSH_TOKEN}
*/
pub fn activity_cron(authentication_token: &str) {
    // 1. Remove all tokens that have not been updated for 8 hours
    let conn = conn_db_ios_activity_push_token();

    {
        let mut stmt = conn
            .prepare(
                "DELETE FROM ios_activity_push_token WHERE lastDate < datetime('now', '-12 hours')",
            )
            .unwrap();
        stmt.execute([]).unwrap();
    }

    // 2. Set meal type & date based on current time
    // 0am ~ 8:40am -> breakfast
    // 8:31am ~ 1:50pm -> lunch
    // 1:41pm ~ 7:50pm -> dinner
    // 7:41pm ~ 11:59pm -> breakfast (next day)
    let mut date = chrono::Local::now();

    let now = chrono::Local::now();
    let now_hour = now.hour();
    let now_minute = now.minute();

    let meal_type: &str;

    if now_hour < 8 || (now_hour == 8 && now_minute < 40) {
        meal_type = "breakfast";
    } else if now_hour < 14 || (now_hour == 14 && now_minute < 50) {
        meal_type = "lunch";
    } else if now_hour < 20 || (now_hour == 20 && now_minute < 50) {
        meal_type = "dinner";
    } else {
        meal_type = "breakfast";
        // date = date + 1
        date += chrono::Duration::days(1);
    };

    // 3. Get meal data from database
    let mut meal_data = Meal {
        idx: 0,
        id: 0,
        date: String::from(""),
        breakfast: String::from(""),
        lunch: String::from(""),
        dinner: String::from(""),
    };

    {
        let mut stmt = conn.prepare("SELECT * FROM meals WHERE date = ?").unwrap();
        let mut rows = stmt.query(&[&date.format("%Y-%m-%d").to_string()]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            meal_data = Meal {
                idx: row.get(0).unwrap(),
                id: row.get(1).unwrap(),
                date: row.get(2).unwrap(),
                breakfast: row.get(3).unwrap(),
                lunch: row.get(4).unwrap(),
                dinner: row.get(5).unwrap(),
            };
        }
    }

    // 4. Get all push tokens from database
    let mut ios_tokens: Vec<IosActivityPushToken> = Vec::new();

    {
        let mut stmt = conn
            .prepare("SELECT lastDate, pushToken FROM ios_activity_push_token")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            ios_tokens.push(IosActivityPushToken {
                last_date: row.get(0).unwrap(),
                push_token: row.get(1).unwrap(),
            });
        }
    }

    // 5. Send push notification to all push tokens
    for ios_token in ios_tokens {
        send_activity_notification(
            authentication_token,
            &ios_token.push_token,
            meal_type,
            &serde_json::to_string(&meal_data).unwrap(),
            &date.format("%Y-%m-%d").to_string(),
        );
    }

    conn.close().unwrap();

    println!("Cron job done");
}

// Sends a push notification
pub fn send_activity_notification(
    authentication_token: &str,
    push_token: &str,
    meal_type: &str,
    meal_data: &str,
    date: &str,
) {
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://api.development.push.apple.com/3/device/{}",
        push_token
    );

    let data = json!({
        "aps": {
            "event": "update",
            "content-state": {
                "type": meal_type,
                "meal": meal_data,
                "date": date,
            },
            "timestamp": chrono::Local::now().timestamp(),
        },
    });

    let res = client
        .post(&url)
        .header("authorization", format!("bearer {}", authentication_token))
        .header("apns-topic", "kr.isamin.dimigomeal.push-type.liveactivity")
        .header("apns-push-type", "liveactivity")
        .header("apns-priority", "10")
        .header("apns-expiration", "0")
        .json(&data)
        .send()
        .expect("Failed to send push notification");

    println!("{:?}", res);
}
