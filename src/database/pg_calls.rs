use crate::database::pg_conn::pg_connection;
use crate::lib_models::MediaWeb;
use crate::models::{Media, MediaOriginalPathUpdate, MediaTag, NewMedia, Person, Tag};
use crate::schema::*;
use diesel::associations::HasTable;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use leptos::prelude::ServerFnError;

pub fn return_all() -> QueryResult<usize> {
    let connection = &mut pg_connection();
    //left join media_tags on media.id = media_tags.media_id
    let media_view = media::table
        .left_outer_join(media_tags::table.on(media::id.eq(media_tags::media_id)))
        .left_outer_join(tags::table.on(media_tags::tag_id.eq(tags::id)))
        .select((Media::as_select(), Option::<(Tag)>::as_select()))
        .order(media::id)
        .limit(4)
        .load::<(Media, Option<Tag>)>(connection)
        .expect("Error loading media assets");
    println!("media_with_tags: {:?}", media_view);

    Ok(0)
}

pub fn associate_media_tags() -> QueryResult<usize> {
    let connection = &mut pg_connection();
    //let one_asset = media::table
    //    .select(Media::as_select())
    //    .get_results(connection)?;
    let all_assets = media::table.load::<Media>(connection)?;
    let all_tags = tags::table.load::<Tag>(connection)?;
    let media_mapping = MediaTag::belonging_to(&all_assets).load::<MediaTag>(connection)?;
    let tag_mapping = MediaTag::belonging_to(&all_tags).load::<MediaTag>(connection)?;
    let tags = all_tags.into_iter().zip(&tag_mapping).collect::<Vec<_>>();
    let media = all_assets
        .into_iter()
        .zip(&media_mapping)
        .collect::<Vec<_>>();
    let combo = media.into_iter().zip(tags).collect::<Vec<_>>();

    for row in combo {
        println!("media: {:?}", row);
    }
    Ok(0)
}

pub fn fetch_all_media_assets() -> Vec<Media> {
    use crate::schema::media::dsl::*;
    let connection = &mut pg_connection();
    let results = media
        .filter(reviewed.eq(true))
        .filter(usable.eq(true))
        .filter(media_type.eq("video"))
        .limit(10)
        .select(Media::as_select())
        .order(duration_ms.desc())
        .load(connection)
        .expect("Error loading media assets");
    results
}

pub fn insert_new_media(new_media: &NewMedia) -> QueryResult<usize> {
    use crate::database::pg_conn::pg_connection;
    use crate::schema::media::dsl::*;
    let connection = &mut pg_connection();
    let result = insert_into(media).values(new_media).execute(connection);
    if let Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) = result {
        Ok(0)
    } else {
        result
    }
}

pub fn get_media_tags(media_id: i32) -> Vec<String> {
    let connection = &mut pg_connection();

    let get_tag = media_tags::table
        .left_outer_join(tags::table.on(media_tags::tag_id.eq(tags::id)))
        .filter(media_tags::media_id.eq(media_id))
        .select(tags::name.nullable())
        .load::<Option<String>>(connection);
    if let Ok(tags) = get_tag {
        tags.into_iter().filter_map(|tag| tag).collect()
    } else {
        vec![]
    }
}

pub fn get_media_people(media_id: i32) -> Vec<String> {
    let connection = &mut pg_connection();

    let get_people = media_people::table
        .left_outer_join(people::table.on(media_people::person_id.eq(people::id)))
        .filter(media_people::media_id.eq(media_id))
        .select(people::name.nullable())
        .load::<Option<String>>(connection);
    if let Ok(people) = get_people {
        people.into_iter().filter_map(|person| person).collect()
    } else {
        vec![]
    }
}

pub fn fetch_assets_for_review() -> Vec<Media> {
    use crate::schema::media::dsl::*;
    let connection = &mut pg_connection();
    let results = media
        .filter(media_type.eq("video"))
        .filter(reviewed.eq(false))
        .filter(usable.eq(true))
        .limit(10)
        .select(Media::as_select())
        .order(created_at.asc())
        .load(connection)
        .expect("Error loading media assets");
    results
}

pub async fn fetch_tag_id(tag: String) -> Result<Tag, Error> {
    let connection = &mut pg_connection();
    let tag = tags::table.filter(tags::name.eq(tag)).first(connection);
    tag
}

pub async fn fetch_person_id(person: String) -> Result<Person, Error> {
    let connection = &mut pg_connection();
    let person = people::table
        .filter(people::name.eq(person))
        .first(connection);
    person
}

pub async fn fetch_all_tags() -> Result<Vec<Tag>, Error> {
    let connection = &mut pg_connection();
    let tags = tags::table.load(connection);
    tags
}

pub async fn fetch_all_people() -> Result<Vec<Person>, Error> {
    let connection = &mut pg_connection();
    let people = people::table.load(connection);
    people
}

pub async fn fetch_video_assets(only_unreviewed: bool) -> Result<Vec<MediaWeb>, ServerFnError> {
    //   let ass = associate_media_tags();
    //create MediaView struct
    let assets = if only_unreviewed {
        fetch_assets_for_review()
    } else {
        fetch_all_media_assets()
    };

    let web_assets = assets
        .iter()
        .map(|asset| MediaWeb {
            id: asset.id,
            route: asset.route.clone(),
            root_path: asset.root_path.clone(),
            file_path: asset.file_path.clone(),
            file_name: asset.file_name.clone(),
            media_type: asset.media_type.clone(),
            usable: asset.usable,
            highlight: asset.highlight,
            reviewed: asset.reviewed,
            created_at: asset.created_at,
            uploaded_at: asset.uploaded_at,
            description: asset.description.clone(),
            tags: get_media_tags(asset.id),
            people: get_media_people(asset.id),
        })
        .collect();
    Ok(web_assets)
}

pub async fn search_media_assets(search_string: &str) -> Result<Vec<MediaWeb>, ServerFnError> {
    let connection = &mut pg_connection();
    let search_pattern = format!("%{}%", search_string);

    let results = media::table
        .left_outer_join(media_tags::table.on(media::id.eq(media_tags::media_id)))
        .left_outer_join(tags::table.on(media_tags::tag_id.eq(tags::id)))
        .left_outer_join(media_people::table.on(media::id.eq(media_people::media_id)))
        .left_outer_join(people::table.on(media_people::person_id.eq(people::id)))
        .filter(media::usable.eq(true))
        .filter(
            media::description
                .ilike(&search_pattern)
                .or(tags::name.ilike(&search_pattern))
                .or(people::name.ilike(&search_pattern)),
        )
        .select((
            Media::as_select(),
            Option::<Tag>::as_select(),
            Option::<Person>::as_select(),
        ))
        .load::<(Media, Option<Tag>, Option<Person>)>(connection)
        .map_err(|_| ServerFnError::new("Error searching media assets"))?;

    let mut media_map = std::collections::HashMap::new();

    for (media, tag, person) in results {
        let entry = media_map.entry(media.id).or_insert_with(|| MediaWeb {
            id: media.id,
            route: media.route.clone(),
            root_path: media.root_path.clone(),
            file_path: media.file_path.clone(),
            file_name: media.file_name.clone(),
            description: media.description.clone(),
            tags: vec![],
            people: vec![],
            media_type: media.media_type.clone(),
            usable: media.usable,
            highlight: media.highlight,
            reviewed: media.reviewed,
            created_at: media.created_at,
            uploaded_at: media.uploaded_at,
        });

        if let Some(tag) = tag {
            if !entry.tags.contains(&tag.name) {
                entry.tags.push(tag.name);
            }
        }

        if let Some(person) = person {
            if !entry.people.contains(&person.name) {
                entry.people.push(person.name);
            }
        }
    }

    Ok(media_map.into_values().collect())
}

pub fn get_file_paths_by_ids(media_ids: Vec<i32>) -> Result<Vec<String>, diesel::result::Error> {
    use crate::schema::media::dsl::*;
    let connection = &mut pg_connection();
    let results = media
        .filter(id.eq_any(media_ids)) // Filter by the provided IDs
        .select(diesel::dsl::sql::<diesel::sql_types::Text>(
            "COALESCE(original_path, file_path)",
        )) // Select original_path if not null, otherwise file_path
        .load::<String>(connection)?; // Load the results as a vector of Strings

    Ok(results)
}

pub fn update_media_original_path(
    file_name_: &str,
    root_path_: &str,
    original_path_: String,
) -> Result<(), diesel::result::Error> {
    use crate::schema::media::dsl::*;
    let conn = &mut pg_connection();

    // Get the base name without extension
    let base_name = file_name_
        .rsplit_once('.')
        .map(|(base, _)| base)
        .unwrap_or(file_name_);

    let update = MediaOriginalPathUpdate {
        original_path: Some(original_path_),
    };

    // Use a more precise pattern that ensures we match the exact base name
    diesel::update(media)
        .filter(
            file_name
                .like(format!("{}.%", base_name))
                .and(root_path.eq(root_path_))
                // Exclude any files that start with ._
                .and(file_name.not_like("._%.%")),
        )
        .set(&update)
        .execute(conn)?;

    Ok(())
}
