use rusqlite::Error;

use rusqlite::Connection;

pub mod activity {
    static db_name: &'static str = "ios_activity_key";

    pub struct IosActivityKey {
        pub lastDate: String,
        pub activityKey: String,
    }

    pub fn conn_db_ios_activity_key() -> Connection {
        let db_path = "./db.db3";
        let conn = Connection::open(db_path).unwrap();

        conn
    }

    // key format is A-Z0-9, 8-4-4-4-12 (32 characters)
    pub fn check_key_format(key: &str) -> bool {
        let key = key.to_uppercase();
        let key = key.replace("-", "");
        let key = key.replace(" ", "");

        if key.len() != 32 {
            return false;
        }

        for c in key.chars() {
            if !c.is_ascii_alphanumeric() {
                return false;
            }
        }

        true
    }

    // GET /ios/activity?key=E261988D-0980-458E-8131-90280E1DE952
    #[get("/ios/activity")]
    pub async fn get_ios_activity_key(
        query: web::Query<HashMap<String, String>>,
    ) -> impl Responder {
        let key = query.get("key").unwrap();

        if !check_key_format(key) {
            return HttpResponse::BadRequest().body("Invalid key format");
        }

        let conn = conn_db_ios_activity_key();
        let mut stmt = conn
            .prepare("SELECT lastDate, activityKey FROM ios_activity_key WHERE activityKey = ?")
            .unwrap();
        let mut rows = stmt.query(&[key]).unwrap();

        let mut activity_key = IosActivityKey {
            lastDate: String::from(""),
            activityKey: String::from(""),
        };

        while let Some(row) = rows.next().unwrap() {
            activity_key = IosActivityKey {
                lastDate: row.get(0).unwrap(),
                activityKey: row.get(1).unwrap(),
            };
        }

        if activity_key.activityKey == "" {
            return HttpResponse::NotFound().body("Key not found");
        }

        HttpResponse::Ok().json(activity_key)
    }

    // POST /ios/activity with JSON body {key: String}
    #[post("/ios/activity")]
    pub async fn post_ios_activity_key(data: web::Json<IosActivityKey>) -> impl Responder {
        // get current date format with string YYYY-MM-DD HH:MM:SS
        let current_date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        if !check_key_format(&data.activityKey) {
            return HttpResponse::BadRequest().body("Invalid key format");
        }

        let conn = conn_db_ios_activity_key();
        let mut stmt = conn
            .prepare("SELECT lastDate, activityKey FROM ios_activity_key WHERE activityKey = ?")
            .unwrap();
        let mut rows = stmt.query(&[&data.activityKey]).unwrap();

        let mut activity_key = IosActivityKey {
            lastDate: String::from(""),
            activityKey: String::from(""),
        };

        while let Some(row) = rows.next().unwrap() {
            activity_key = IosActivityKey {
                lastDate: row.get(0).unwrap(),
                activityKey: row.get(1).unwrap(),
            };
        }

        if activity_key.activityKey == "" {
            let mut stmt = conn
                .prepare("INSERT INTO ios_activity_key (lastDate, activityKey) VALUES (?, ?)")
                .unwrap();
            stmt.execute(&[&current_date, &data.activityKey]).unwrap();
        } else {
            let mut stmt = conn
                .prepare("UPDATE ios_activity_key SET lastDate = ? WHERE activityKey = ?")
                .unwrap();
            stmt.execute(&[&current_date, &data.activityKey]).unwrap();
        }

        HttpResponse::Ok().json(IosActivityKey {
            lastDate: current_date,
            activityKey: data.activityKey.clone(),
        })
    }
}
