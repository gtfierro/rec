/*
 * Simple query language that is just passthrough to sqlite3
 * rec query --collections <col1> <col2> <query>
 * rec query <collection> <col1> <query>
 *
 * query:
 * < 3.4
 * > 4.5
 * <= 700
 * => 800
 * == .9
 * and
 * or
 */

pub fn query_to_sql(collection: String, tokens: Vec<String>) -> String {
    let mut query = String::from(format!("SELECT collection, time, value FROM data WHERE collection = '{}' and ", collection));
    let mut tokeniterator = tokens.iter();
    loop {
        match tokeniterator.next() {
            Some(token) => {
                match token.to_lowercase().as_ref() {
                    op @ "<" | op @ ">" | op @ "<=" | op @ ">=" => {
                        match tokeniterator.next() {
                            Some(valuetoken) => {
                                query.push_str(&format!(" value {} {} ", op, valuetoken));
                            },
                            None => { break }
                        }
                    }
                    "and" => { query.push_str(" and "); },
                    "or" => { query.push_str(" or "); },
                    val => { println!("value: {}", val); }
                }
            },
            None => { break }

        }
    }
    query
}
