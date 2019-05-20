use crate::chapters::example;
use gremlin_client::process::traversal::GraphTraversalSource;

fn chapter_312(g: &GraphTraversalSource) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.12";

    example(
        &g,
        chapter,
        "Sort the first 20 airports returned in ascending order",
        |g| {
            let results = g
                .v(())
                .has_label("airport")
                .limit(20)
                .values("code")
                .order(())
                .fold()
                .to_list()?;
            Ok(format!("Found {:?} ", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "Sort all of the airports in the graph by their code and then return the first 20",
        |g| {
            let results = g
                .v(())
                .has_label("airport")
                .order(())
                .by("code")
                .limit(20)
                .values("code")
                .fold()
                .to_list()?;

            Ok(format!("Found {:?} ", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "Sort all of the places you can fly to from Austin (AUS)",
        |g| {
            let results = g
                .v(())
                .has(("code", "AUS"))
                .out(())
                .order(())
                .by("code")
                .values(vec!["code", "icao"])
                .fold()
                .to_list()?;

            Ok(format!("Found {:?} ", results[0]))
        },
    )?;

    Ok(())
}

pub fn all() -> Vec<Box<Fn(&GraphTraversalSource) -> Result<(), Box<std::error::Error>>>> {
    vec![Box::new(chapter_312)]
}
