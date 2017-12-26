extern crate mysql;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Connection {
    pool: mysql::Pool
}

pub struct Matrix {
    pub rows: Vec<HashMap<String, String>>,
    pub column_names: Vec<String>
}

impl Connection {
    pub fn new(url: &String) -> Result<Connection, String> {
        let pool = mysql::Pool::new(url).unwrap();

        Ok(Connection {
            pool
        })
    }

    pub fn query(&mut self, query: &String) -> Matrix {
        let con = &mut self.pool.get_conn().expect("Failed to retrieve a connection");
        let result = con.query(query).expect(&format!("Failed to execute query: {}", &query));

        let column_names = Connection::extract_column_names(&result);
        println!("{:?}", column_names);

        let mut rows = Vec::new();
        for result_row in result {
            let mut row_hash_map = HashMap::new();
            let row: mysql::Row = result_row.expect("lol");
            let mut index = 0;
            for col in column_names.clone() {
                let value: String = match row.get(index) {
                    Some(val) => val,
                    None => panic!("lol"),
                };
                row_hash_map.insert(col, value);
                index = index + 1;
            }
            rows.push(row_hash_map);
        }
        println!("{:?}", rows);
        Matrix {rows, column_names}
    }

    fn extract_column_names(qr: &mysql::QueryResult) -> Vec<String> {
        let mut column_names = Vec::new();
        for col in qr.columns_ref() {
            column_names.push(col.name_str().to_string());
        }
        column_names
    }
}
