fn save() {
    let record = sqlx::query!(
        r#"INSERT INTO catalog ( path ,display_name,short_desc, long_desc, created_at, updated_at ) VALUES ( $1, $2, $3, $4, $5, $6) RETURNING id"#,
        "/test",
        "display_name",
        "short_desc",
        "long_desc",
        time::OffsetDateTime::now_utc(),
        time::OffsetDateTime::now_utc()
    )
    .fetch_one(&database)
    .await?;
    println!("{}", record.id);

    let catalog = sqlx::query_as!(Catalog, r#"SELECT * FROM catalog WHERE id = $1"#, record.id)
        .fetch_one(&database)
        .await?;
    println!("{catalog:?}");

    let record = sqlx::query!(
        r#"INSERT INTO video ( path, display_name, short_desc, long_desc, catalog_id, sequent_id, size, duration, bitrate, width, height, framerate ) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id"#,
        "/test",
        "display_name",
        "short_desc",
        "long_desc",
        1,
        None as Option<ModelId>,
        100000,
        2400,
        120,
        1280,
        1024,
        60.
    )
    .fetch_one(&database)
    .await?;
    println!("{}", record.id);
}
