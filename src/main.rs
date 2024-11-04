use rusqlite::{params, Connection, Result};
use std::time::Instant;
use sysinfo::{System, ProcessesToUpdate};


fn query() -> Result<String> {
    let conn = Connection::open("US_births_DB.db")?;

    // C(reate)
    conn.execute(
        "INSERT INTO US_births_DB (year, month, date_of_month, day_of_week, births)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![2024, 10, 5, 6, 7785],
    )?;

    // R(ead)
    let mut stmt = conn.prepare("SELECT * FROM US_births_DB")?;
    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let year: i32 = row.get(1)?;
        let month: i32 = row.get(2)?;
        let date_of_month: i32 = row.get(3)?;
        let day_of_week: i32 = row.get(4)?;
        let births: i32 = row.get(5)?;
        Ok((id, year, month, date_of_month, day_of_week, births))
    })?;

    let mut count = 0;
    for row in rows {
        if count >= 20 {
            break;
        }
        println!("{:?}", row?);
        count += 1;
    }

    // U(pdate)
    conn.execute(
        "UPDATE US_births_DB SET births = ?1 WHERE id = ?2",
        params![7899, 55],
    )?;

    // D(elete)
    conn.execute("DELETE FROM US_births_DB WHERE id = ?1", params![420])?;

    Ok("Success!".to_string())
}


fn main() {
    let start = Instant::now();

    let mut sys = System::new_all();
    sys.refresh_all();

    let result = query();

    let elapsed = start.elapsed();
    println!("Execution time: {:.2?}", elapsed);

    sys.refresh_processes(ProcessesToUpdate::All, true);
    if let Some(process) = sys.process(sysinfo::get_current_pid().unwrap()) {
        println!("Memory usage: {} KB", process.memory());
        println!("CPU usage: {}%", process.cpu_usage());
    }

    match result {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Error: {}", e),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        assert_eq!(query().unwrap(), "Success!");
    }
}
