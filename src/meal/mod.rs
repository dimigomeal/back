use chrono::{prelude::*, Duration};
use regex::Regex;
use rusqlite::Connection;
use serde::Serialize;
use std::io::Error;
use visdom::Vis;

#[derive(Serialize)]
pub struct Meal {
    pub idx: i32,
    pub id: i32,
    pub date: String,
    pub breakfast: String,
    pub lunch: String,
    pub dinner: String,
}

pub async fn conn_db_meals() -> Result<Connection, Error> {
    let db_path: &str = "./db.db3";
    let conn: Connection;

    {
        conn = Connection::open(db_path).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS meals (
                idx INTEGER PRIMARY KEY AUTOINCREMENT,
                id INTEGER NOT NULL,
                date TEXT NOT NULL,
                breakfast TEXT,
                lunch TEXT,
                dinner TEXT
            )",
            [],
        )
        .unwrap();
    }

    Ok(conn)
}

pub async fn get_meal(date: &str) -> Result<Meal, Error> {
    let conn = conn_db_meals().await.unwrap();
    let meal: Meal;

    {
        let mut stmt = conn.prepare("SELECT * FROM meals WHERE date = ?").unwrap();
        let mut rows = stmt
            .query_map([date], |row| {
                Ok(Meal {
                    idx: row.get(0)?,
                    id: row.get(1)?,
                    date: row.get(2)?,
                    breakfast: row.get(3)?,
                    lunch: row.get(4)?,
                    dinner: row.get(5)?,
                })
            })
            .unwrap();

        match rows.next() {
            None => {
                return Err(Error::new(std::io::ErrorKind::NotFound, "Meal not found"));
            }
            row => {
                meal = row.unwrap().unwrap();
            }
        }
    }

    conn.close().unwrap();

    Ok(meal)
}

pub async fn get_meal_week(date: &str) -> Result<Vec<Meal>, Error> {
    let conn = conn_db_meals().await.unwrap();
    let mut meals = Vec::new();

    let start_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap() - Duration::days(6);
    let start_date_str = start_date.format("%Y-%m-%d").to_string();
    let end_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap() + Duration::days(6);
    let end_date_str = end_date.format("%Y-%m-%d").to_string();

    {
        let mut stmt = conn
            .prepare("SELECT * FROM meals WHERE date BETWEEN ?1 AND ?2")
            .unwrap();
        let rows = stmt
            .query_map([start_date_str, end_date_str], |row| {
                Ok(Meal {
                    idx: row.get(0)?,
                    id: row.get(1)?,
                    date: row.get(2)?,
                    breakfast: row.get(3)?,
                    lunch: row.get(4)?,
                    dinner: row.get(5)?,
                })
            })
            .unwrap();

        for meal in rows {
            meals.push(meal.unwrap());
        }
    }

    conn.close().unwrap();

    Ok(meals)
}

fn parse_string(string: &str) -> String {
    let basic_regex = Regex::new(r"<(?:.|\n)*?>").unwrap();
    basic_regex
        .replace_all(string, "")
        .to_string()
        .trim()
        .replace("\t", "")
        .replace(" ", "")
        .replace("*", "")
        .to_string()
}

fn parse_number(string: &str) -> i32 {
    let number_regex = Regex::new(r"\d+").unwrap();
    let number_string = number_regex.find(string).unwrap().as_str();
    number_string.parse().unwrap()
}

fn split_string(string: &str, split: &str) -> Vec<String> {
    string.split(split).map(|s| s.to_string()).collect()
}

fn date_string(year: i32, month: i32, day: i32) -> String {
    let date =
        NaiveDate::from_ymd_opt(year, month.try_into().unwrap(), day.try_into().unwrap()).unwrap();
    date.format("%Y-%m-%d").to_string()
}

pub async fn meal_cron() -> Result<(), Error> {
    let conn = conn_db_meals().await.unwrap();

    let list_res =
        reqwest::get("https://www.dimigo.hs.kr/index.php?mid=school_cafeteria&page=1").await;

    match list_res {
        Ok(res) => {
            let list_html = res.text().await.unwrap();
            let list_ele = Vis::load(list_html).unwrap();

            let key_list = list_ele.find("#siLst thead th");
            let mut index = 0;
            let mut id_index = 0;
            let mut title_index = 0;
            let mut date_index = 0;
            for key in key_list {
                let key_string = parse_string(&key.text());

                if key_string == "번호" {
                    id_index = index;
                } else if key_string == "제목" {
                    title_index = index;
                } else if key_string == "등록일" {
                    date_index = index;
                }

                index += 1;
            }

            println!("ID Index: {}", id_index);
            println!("Title Index: {}", title_index);
            println!("Date Index: {}", date_index);
            println!();

            let item_list = list_ele.find("#siLst tbody tr");
            for item in item_list {
                let value_ele = Vis::load(item.html()).unwrap();
                let value_list = value_ele.find("td");

                let mut index = 0;
                let mut id_value = 0;
                let mut title_value = String::new();
                let mut date_value = String::new();
                let url_value = value_ele.find(".title a").attr("href").unwrap().to_string();
                for value in value_list {
                    let value_string = parse_string(&value.text());

                    if index == id_index {
                        id_value = value_string.parse().unwrap();
                    } else if index == title_index {
                        title_value = value_string;
                    } else if index == date_index {
                        date_value = value_string;
                    }

                    index += 1;
                }

                let title_list = split_string(&title_value, "월");
                if title_list.len() < 2 {
                    continue;
                }

                let date_list = split_string(&date_value, "-");
                if date_list.len() < 3 {
                    continue;
                }

                let year = parse_number(&date_list[0]);
                let month = parse_number(&title_list[0]);
                let day = parse_number(&title_list[1]);
                let date = date_string(year, month, day);

                println!("===== POST =====");
                println!("ID: {}", id_value);
                println!("Title: {}", title_value);
                println!("Date: {}", date_value);
                println!("URL: {}", url_value);

                let item_res = reqwest::get(&url_value).await;
                match item_res {
                    Ok(res) => {
                        let item_html = res.text().await.unwrap();
                        let item_ele = Vis::load(item_html).unwrap();

                        let content = item_ele.find(".scConDoc").text();
                        let meal_list = split_string(&content, "\n");
                        let mut breakfast = String::new();
                        let mut lunch = String::new();
                        let mut dinner = String::new();
                        for meal in meal_list {
                            let meal_string = parse_string(&meal);
                            let meal_split = split_string(&meal_string, ":");
                            if meal_split.len() < 2 {
                                continue;
                            }

                            let meal_key = meal_split[0].to_string();
                            let meal_value = meal_split[1..].join(":").trim().to_string();

                            if meal_key.contains("조식") {
                                breakfast = meal_value;
                            } else if meal_key.contains("중식") {
                                lunch = meal_value;
                            } else if meal_key.contains("석식") {
                                dinner = meal_value;
                            }
                        }

                        println!("===== CONTENT =====");
                        println!("Date: {}", date);
                        println!("Breakfast: {}", breakfast);
                        println!("Lunch: {}", lunch);
                        println!("Dinner: {}", dinner);

                        let exist = conn
                            .query_row(
                                "SELECT COUNT(*) FROM meals WHERE id=?1",
                                [id_value],
                                |row| row.get::<usize, i32>(0),
                            )
                            .unwrap();

                        if exist == 0 {
                            println!("===== INSERT =====");
                            conn.execute(
                                    "INSERT INTO meals (id, date, breakfast, lunch, dinner) VALUES (?1, ?2, ?3, ?4, ?5)",
                                    (id_value, date, breakfast, lunch, dinner),
                                )
                                .unwrap();
                        } else {
                            println!("===== UPDATE =====");
                            conn.execute(
                                    "UPDATE meals SET date=?1, breakfast=?2, lunch=?3, dinner=?4 WHERE id=?5",
                                    (date, breakfast, lunch, dinner, id_value),
                                )
                                .unwrap();
                        }
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }
                println!("");
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    Ok(())
}
