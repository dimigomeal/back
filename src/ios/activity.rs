use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::io::Error;
use std::io::ErrorKind;

use chrono::Timelike;
use rusqlite::Connection;
use serde::Serialize;

use crate::func;
use crate::meal;

#[derive(Serialize)]
pub struct IosActivityDeviceToken {
    pub created_date: String,
    pub device_token: String,
}

pub async fn conn_db_ios_activity_device_tokens() -> Result<Connection, Error> {
    let db_path = "./db.db3";
    let conn: Connection;

    {
        conn = Connection::open(db_path).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS ios_activity_device_tokens (
                createdDate TEXT,
                deviceToken TEXT PRIMARY KEY
            )",
            [],
        )
        .unwrap();
    }

    Ok(conn)
}

pub async fn get_device_token(token: String) -> Result<IosActivityDeviceToken, Error> {
    let conn: Connection = conn_db_ios_activity_device_tokens().await.unwrap();

    let mut ios_token = IosActivityDeviceToken {
        created_date: String::from(""),
        device_token: String::from(""),
    };

    {
        let mut stmt = conn
            .prepare(
                "SELECT createdDate, deviceToken FROM ios_activity_device_tokens WHERE deviceToken = ?",
            )
            .unwrap();
        let mut rows = stmt.query(&[&token]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            ios_token = IosActivityDeviceToken {
                created_date: row.get(0).unwrap(),
                device_token: row.get(1).unwrap(),
            };
        }
    }

    conn.close().unwrap();

    Ok(ios_token)
}

pub async fn add_device_token(token: String) -> Result<IosActivityDeviceToken, Error> {
    let conn: Connection = conn_db_ios_activity_device_tokens().await.unwrap();

    let mut ios_token = IosActivityDeviceToken {
        created_date: String::from(""),
        device_token: String::from(""),
    };

    {
        let mut stmt = conn
            .prepare(
                "SELECT createdDate, deviceToken FROM ios_activity_device_tokens WHERE deviceToken = ?",
            )
            .unwrap();
        let mut rows = stmt.query(&[&token]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            ios_token = IosActivityDeviceToken {
                created_date: row.get(0).unwrap(),
                device_token: row.get(1).unwrap(),
            };
        }
    }

    if ios_token.device_token != "" {
        conn.close().unwrap();
        return Err(Error::new(ErrorKind::Other, "Token already exists"));
    }

    conn.execute(
        "INSERT INTO ios_activity_device_tokens (createdDate, deviceToken) VALUES (datetime('now'), ?)",
        &[&token],
    )
    .unwrap();

    conn.close().unwrap();

    let result = IosActivityDeviceToken {
        created_date: chrono::Local::now().to_string(),
        device_token: token,
    };

    Ok(result)
}

pub async fn remove_device_token(token: String) -> Result<bool, Error> {
    let conn = conn_db_ios_activity_device_tokens().await.unwrap();

    conn.execute(
        "DELETE FROM ios_activity_device_tokens WHERE deviceToken = ?",
        &[&token],
    )
    .unwrap();

    conn.close().unwrap();

    Ok(true)
}

async fn remove_old_device_tokens() -> Result<(), Error> {
    let conn = conn_db_ios_activity_device_tokens().await.unwrap();

    conn.execute(
        "DELETE FROM ios_activity_device_tokens WHERE createdDate < datetime('now', '-8 hours')",
        [],
    )
    .unwrap();

    conn.close().unwrap();

    Ok(())
}

fn get_current() -> (String, String) {
    let mut date = chrono::Local::now();

    let now = chrono::Local::now();
    let now_hour = now.hour();
    let now_minute = now.minute();
    let total_minutes = now_hour * 60 + now_minute;

    let meal_type: &str;

    match total_minutes {
        0..=480 => {
            meal_type = "breakfast";
        }
        481..=810 => {
            meal_type = "lunch";
        }
        811..=1170 => {
            meal_type = "dinner";
        }
        _ => {
            meal_type = "breakfast";
            date += chrono::Duration::days(1);
        }
    }

    (meal_type.to_string(), date.format("%Y-%m-%d").to_string())
}

pub async fn activity_cron(private_key: &str) -> Result<(), Error> {
    remove_old_device_tokens().await.unwrap();

    let (meal_type, date) = get_current();

    let meal = meal::get_meal(&date).await;
    let menu = match meal {
        Ok(meal) => match meal_type.as_str() {
            "breakfast" => meal.breakfast,
            "lunch" => meal.lunch,
            "dinner" => meal.dinner,
            _ => "".to_string(),
        },
        Err(_) => "".to_string(),
    };

    let conn = conn_db_ios_activity_device_tokens().await.unwrap();
    let mut ios_tokens: Vec<IosActivityDeviceToken> = Vec::new();

    {
        let mut stmt = conn
            .prepare("SELECT createdDate, deviceToken FROM ios_activity_device_tokens")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            ios_tokens.push(IosActivityDeviceToken {
                created_date: row.get(0).unwrap(),
                device_token: row.get(1).unwrap(),
            });
        }
    }

    for ios_token in ios_tokens {
        send_activity(
            private_key,
            &ios_token.device_token,
            meal_type.as_str(),
            &menu,
            &date,
        )
        .await
        .unwrap();
    }

    conn.close().unwrap();

    Ok(())
}

pub async fn send_custom_activity(
    private_key: &str,
    device_token: &str,
    meal_type: &str,
    menu: &str,
    date: &str,
) -> Result<(), Error> {
    send_activity(private_key, device_token, meal_type, menu, date).await
}

pub async fn send_activity(
    private_key: &str,
    device_token: &str,
    meal_type: &str,
    menu: &str,
    date: &str,
) -> Result<(), Error> {
    let token: String = func::get_ios_activity_push_token(private_key);

    let body = json!({
        "aps": {
            "event": "update",
            "content-state": {
                "type": meal_type,
                "menu": menu,
                "date": date
            },
            "timestamp": chrono::Local::now().timestamp()
        }
    });

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(format!("bearer {}", token).as_str()).unwrap(),
    );
    headers.insert(
        "apns-topic",
        HeaderValue::from_static("kr.isamin.dimigomeal.push-type.liveactivity"),
    );
    headers.insert("apns-push-type", HeaderValue::from_static("liveactivity"));
    headers.insert("apns-priority", HeaderValue::from_static("10"));
    headers.insert("apns-expiration", HeaderValue::from_static("0"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::ClientBuilder::new()
        .use_rustls_tls()
        .build()
        .unwrap();

    client
        .post(format!(
            "https://api.development.push.apple.com:443/3/device/{}",
            device_token
        ))
        .headers(headers)
        .json(&body)
        .send()
        .await
        .unwrap();

    Ok(())
}
