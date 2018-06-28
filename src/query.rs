/*
 * Simple query language that is just passthrough to sqlite3
 * rec query --collections <col1> <col2> <query>
 * rec query <collection> <col1> <query>
 *
 * query:
 */

pub fn query_to_sql(collection: String, tokens: Vec<String>) -> String {
    let mut query = String::from(format!("SELECT collection, time, value FROM data WHERE collection = '{}' and ", collection));
    let mut tokeniterator = tokens.iter();
    loop {
        match tokeniterator.next() {
            Some(token) => {
                match token.to_lowercase().as_ref() {
                    op @ "<" | op @ ">" | op @ "<=" | op @ ">=" | op @ "=" => {
                        match tokeniterator.next() {
                            Some(valuetoken) => {
                                query.push_str(&format!(" value {} {} ", op, valuetoken));
                            },
                            None => { break }
                        }
                    },
                    op @ "and" | op @ "or" => { query.push_str(&format!(" {} ", op)); }
                    _ => {
                        break
                    }
                }
            },
            None => { break }

        }
    }
    query
}
