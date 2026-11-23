use anyhow::{Result, bail};
use bench_common::{
    database::postgres_url,
    suburbs::{SEARCH_POINT, SRID, SUBURBS},
};
use diesel::prelude::*;
use diesel_demo::crud::{self, AnyConnection};
use postgis_diesel::{
    functions::st_contains,
    types::{Point, Polygon},
};

diesel::table! {
    use diesel::sql_types::Text;
    use postgis_diesel::sql_types::Geometry;

    suburb (name) {
        name -> Text,
        outline -> Geometry,
    }
}

#[derive(Insertable)]
#[diesel(table_name = suburb)]
struct NewSuburb {
    name: &'static str,
    outline: Polygon<Point>,
}

fn point((lon, lat): (f64, f64)) -> Point {
    Point::new(lon, lat, Some(SRID))
}

fn main() -> Result<()> {
    let AnyConnection::Postgres(mut conn) = crud::connect(&postgres_url("talk_diesel"))? else {
        bail!("the geo example needs Postgres");
    };

    diesel::delete(suburb::table).execute(&mut conn)?;
    for s in SUBURBS {
        let mut outline = Polygon::new(Some(SRID));
        outline.add_points(s.outline.iter().copied().map(point))?;
        diesel::insert_into(suburb::table)
            .values(NewSuburb {
                name: s.name,
                outline,
            })
            .execute(&mut conn)?;
    }

    let names: Vec<String> = suburb::table
        .filter(st_contains(suburb::outline, point(SEARCH_POINT)))
        .select(suburb::name)
        .load(&mut conn)?;
    println!("diesel: {SEARCH_POINT:?} is in {names:?}");
    Ok(())
}
