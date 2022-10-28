use std::f64;

use mysql::{prelude::Queryable, OptsBuilder, Pool};

#[derive(Debug, PartialEq)]
pub struct Monitor {
    idc: String,
    host: String,
    cpu: f64,
    memory: f64,
}

fn main() {
    let builder = OptsBuilder::new()
        .ip_or_hostname(Some("127.0.0.1"))
        .tcp_port(4002)
        .prefer_socket(false);
    let pool = Pool::new(builder).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // create table
    let create = r"
CREATE TABLE dist_monitor (
    idc STRING,
    host STRING,
    ts TIMESTAMP DEFAULT current_timestamp(),
    cpu DOUBLE DEFAULT 0,
    memory DOUBLE,
    TIME INDEX (ts),
    PRIMARY KEY(host))
PARTITION BY RANGE COLUMNS (idc) (
    PARTITION p0 VALUES LESS THAN ('host_3000'),
    PARTITION p1 VALUES LESS THAN ('host_6000'),
    PARTITION p2 VALUES LESS THAN (MAXVALUE),
    )
ENGINE=mito;
    ";
    match conn.query_drop(create) {
        Ok(_) => println!("create table success!"),
        Err(e) => println!("create table failed, err: {:?}", e),
    };

    // insert data
    let count = 30000;
    for i in 0..count {
        let insert = format! {"INSERT INTO dist_monitor (idc, host, cpu, memory)
        VALUES (\"idc_{}\", \"host_{}\", 0.2, 0.3)", i, i};
        match conn.query_drop(insert) {
            Ok(_) => println!("insert success"),
            Err(e) => println!("insert failed, err: {:?}", e),
        }
    }
}
