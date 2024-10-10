

fn add_book(
    conn: &Connection,
    rnd: &mut ThreadRng,
    name: impl AsRef<str>,
    author: impl AsRef<str>,
    publish_date: NaiveDate,
) -> ErrorResult {
    let publish_date_str = publish_date.to_string();
    let id = rnd.gen::<u32>();

    conn.execute(
        "insert into Books (ID,NAME,AUTHOR,PUBLISH_DATE) values (?1, ?2, ?3,?4)",
        params![id, name.as_ref(), author.as_ref(), publish_date_str],
    )?;
    Ok(())
}

pub fn sql() -> ErrorResult {
    let conn = Connection::open_in_memory()?;
    let mut rnd = rand::thread_rng();
    conn.execute(
        "create table if not exists Books (
        ID int unsigned primary key unique default 0,
        NAME text not null,
        AUTHOR text not null,...
        PUBLISH_DATE date not null
    )",
        (),
    )?;

    add_book(
        &conn,
        &mut rnd,
        "try",
        "test",
        NaiveDate::from_ymd_opt(2024, 10, 7).unwrap(),
    )?;

    add_book(
        &conn,
        &mut rnd,
        "hello",
        "world",
        NaiveDate::from_ymd_opt(2023, 10, 7).unwrap(),
    )?;

    let mut query = conn.prepare("select ID,NAME,AUTHOR,PUBLISH_DATE,ROWID from Books")?;
    let vec: Vec<Book> = query
        .query_map((), |row| {
            let date: String = row.get_unwrap(3);
            Ok(Book {
                rowid: row.get(4)?,
                id: row.get(0)?,
                name: row.get(1)?,
                author: row.get(2)?,
                publish_date: NaiveDate::from_str(date.as_str()).unwrap(),
            })
        })?
        .map(Result::unwrap)
        .collect();
    for book in vec {
        println!(
            "ID: {} | NAME: {} AUTHOR: {} | PUBLISH_DATE: {} | ROWID: {}",
            book.id, book.name, book.author, book.publish_date, book.rowid
        )
    }

    Ok(())
}
