use std::io::Write;

use chrono::{Duration, NaiveTime};
use mysql::prelude::*;
use mysql::*;

#[derive(Clone)]
pub enum Mode {
    Punc,
    Calc,
}

pub fn punch_entry(
    tag: String,
    mode: Mode,
) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut param_vec: Vec<String> = Vec::new();
    let time_now = chrono::offset::Local::now().to_string()[0..19].to_string();
    let url = "mysql://root:F00z@804@localhost:3306/PNCHCRD";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;
    let name = String::from("None");

    let query = "SELECT NAME FROM USER WHERE TAG = :tag";
    let output: Vec<Option<String>> = conn.exec(
        query,
        params! {
            "tag" => tag.clone(),
        },
    )?;

    if let Some(Some(name_get)) = output.get(0) {
        param_vec.push(name_get.to_string());
    } else {
        param_vec.push(name);
    }
    param_vec.push(tag.clone());
    param_vec.push(time_now[0..19].to_string());

    let query = "SELECT PERMAID FROM USER WHERE TAG = :tag";

    let output: Vec<Option<u8>> = conn.exec(
        query,
        params! {
            "tag" => tag.clone()
        },
    )?;

    if let Some(Some(permaid)) = output.get(0) {
        //CALCULATE TIME DIFF
        let mut in_times: Vec<String> = Vec::new();
        let mut out_times: Vec<String> = Vec::new();

        let query = "SELECT DIRECTION FROM LOG WHERE PERMAID = :permaid ORDER BY LOGID DESC";
        let output: Vec<Option<String>> = conn.exec(
            query,
            params! {
                "permaid" => permaid
            },
        )?;

        match mode {
            Mode::Punc => {
                if let Some(Some(direction)) = output.get(0) {
                    if *direction.trim().to_string() == String::from("IN") {
                        println!("OUT");
                        param_vec.push(String::from("OUT"));
                        let query =
                    "INSERT INTO LOG (PERMAID,PNCH,DIRECTION) VALUES (:permaid,:pnch,:direction)";
                        conn.exec_drop(
                            query,
                            params! {
                                "permaid" => permaid,
                                "pnch" => time_now[0..19].to_string(),
                                "direction" => String::from("OUT"),
                            },
                        )?;
                    } else {
                        println!("IN");
                        param_vec.push(String::from("IN"));
                        let query =
                    "INSERT INTO LOG (PERMAID,PNCH,DIRECTION) VALUES (:permaid,:pnch,:direction)";
                        conn.exec_drop(
                            query,
                            params! {
                                "permaid" => permaid,
                                "pnch" => time_now[0..19].to_string(),
                                "direction" => String::from("IN"),
                            },
                        )?;
                    }
                } else {
                    println!("IN");
                    param_vec.push(String::from("IN"));
                    let query =
                "INSERT INTO LOG (PERMAID,PNCH,DIRECTION) VALUES (:permaid,:pnch,:direction)";
                    conn.exec_drop(
                        query,
                        params! {
                            "permaid" => permaid,
                            "pnch" => time_now[0..19].to_string(),
                            "direction" => String::from("IN"),
                        },
                    )?;
                }
            }
            Mode::Calc => {
                let mut permaid = String::new();
                print!("Permaid: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin()
                    .read_line(&mut permaid)
                    .expect("Failed to read user input!");
                let mut time_now = String::new();
                print!("Date: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin()
                    .read_line(&mut time_now)
                    .expect("Failed to read user input!");

                let query = "SELECT DATE_FORMAT(PNCH, '%H:%i:%s') AS PNCH, DIRECTION FROM LOG WHERE PERMAID = :permaid AND DATE(PNCH) = :date ORDER BY PNCH ASC";
                let times: Vec<(String, String)> = conn.exec_map(
                    query,
                    params! {"permaid" => permaid, "date" => time_now},
                    |(t_time, t_direction)| (t_time, t_direction),
                )?;

                let mut time_diff = Duration::zero();
                for i in times {
                    if i.1 == String::from("IN") {
                        in_times.push(i.0);
                    } else {
                        out_times.push(i.0);
                    }
                }
                println!("IN: {:?} OUT: {:?}", in_times, out_times);

                if in_times.len() > out_times.len() {
                    for (i, k) in out_times.iter().enumerate() {
                        let dt1 = NaiveTime::parse_from_str(k.as_str(), "%H:%M:%S").unwrap();
                        let dt2 =
                            NaiveTime::parse_from_str(in_times[i].as_str(), "%H:%M:%S").unwrap();
                        time_diff = time_diff + (dt1 - dt2);
                    }
                } else {
                    for (i, k) in in_times.iter().enumerate() {
                        let dt1 = NaiveTime::parse_from_str(k.as_str(), "%H:%M:%S").unwrap();
                        let dt2 =
                            NaiveTime::parse_from_str(out_times[i].as_str(), "%H:%M:%S").unwrap();
                        time_diff = time_diff + (dt2 - dt1);
                    }
                }
                println!("{:?}", time_diff);
            }
        }
    }
    Ok(param_vec)
}
