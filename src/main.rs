use aws_smithy_types_convert::date_time::DateTimeExt;
use chrono::Utc;
use std::{convert::TryFrom, time::SystemTime};

fn main() {
    aws_smithy_types::date_time::DateTime::from_secs(5).to_chrono_utc(); // works

    let ec2_time = aws_sdk_ec2::types::DateTime::from_secs(5);
    ec2_time.to_chrono_utc(); // doesn't work;no method found

    // works
    let sys_time = SystemTime::try_from(ec2_time).unwrap();
    let chrono_time = chrono::DateTime::<Utc>::from(sys_time);
}
