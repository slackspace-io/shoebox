use diesel::prelude::*;

//remove tag associaton with a media_id and string tag
pub async fn remove_tag(media_id: i32, tag: String) -> QueryResult<usize> {
    //get tag_id from tag string
    use crate::database::pg_conn::pg_connection;
    use crate::schema::media_tags;
    let connection = &mut pg_connection();
    let result = crate::database::pg_calls::fetch_tag_id(tag).await;
    let tag = match result {
        Ok(tag) => tag,
        Err(e) => {
            println!("Error fetching tag_id: {}", e);
            return Ok(0);
        }
    };
    let removal = diesel::delete(
        media_tags::table.filter(
            media_tags::media_id
                .eq(media_id)
                .and(media_tags::tag_id.eq(tag.id)),
        ),
    )
    .execute(connection);
    match removal {
        Ok(_) => {
            println!("Tag removed successfully");
        }
        Err(e) => {
            println!("Error removing tag: {}", e);
        }
    }

    Ok(0)
}

//remove tag associaton with a media_id and string tag
pub async fn remove_person(media_id: i32, person: String) -> QueryResult<usize> {
    //get tag_id from tag string
    use crate::database::pg_conn::pg_connection;
    use crate::schema::media_people;
    let connection = &mut pg_connection();
    let result = crate::database::pg_calls::fetch_person_id(person).await;
    let person = match result {
        Ok(person) => person,
        Err(e) => {
            println!("Error fetching person_id: {}", e);
            return Ok(0);
        }
    };
    let removal = diesel::delete(
        media_people::table.filter(
            media_people::media_id
                .eq(media_id)
                .and(media_people::person_id.eq(person.id)),
        ),
    )
    .execute(connection);
    match removal {
        Ok(_) => {
            println!("Person removed successfully");
        }
        Err(e) => {
            println!("Error removing person: {}", e);
        }
    }

    Ok(0)
}
