use std::{
    fs::OpenOptions, io::{self, Write}, path::PathBuf, time::Instant
};

use axum::http::StatusCode;
use chrono::Utc;

use crate::core::enums::log_status_enum::LogStatusEnum;

/**
 * FORMAT :
 * current_time | log_status | http_response_status_code | http_method | endpoint | request_header | request_body | request_params | response_body/error_message | process_taken_time
 */

#[derive(Debug)]
pub struct Log { 
    pub current_time: Instant,
    pub log_status: LogStatusEnum,
    pub status_code: Option<StatusCode>,
    pub http_method: String,
    pub endpoint: String,
    pub request_header: String,
    pub request_body: Option<String>,
    pub request_params: Option<String>,
    pub response_body: Option<String>,
    pub process_taken_time: Option<u128>,
}

impl Log {
    pub fn request(
        http_method: &String,
        endpoint: &String,
        request_header: &String,
        body: Option<String>
    ) -> io::Result<()> {
        let log: Log = Self {
            current_time: Instant::now(),
            log_status: LogStatusEnum::Info,
            status_code: None,
            http_method: http_method.clone(),
            endpoint: format!("{:?}", endpoint),
            request_header: format!("{:?}", request_header),
            request_body: body.map(|s| s.to_string()),
            request_params: None,
            response_body: None,
            process_taken_time: None,
        };

        log.write_to_file()
    }

    pub fn response(
        http_method: String,
        endpoint: String,
        request_header: String,
        status: StatusCode,
        body: Option<String>,
        response_time: u128,
    ) -> io::Result<()> {
        let log = Self {
            current_time: Instant::now(),
            log_status: if status == StatusCode::OK || status == StatusCode::CREATED {
                LogStatusEnum::Verbose
            } else if status == StatusCode::BAD_REQUEST {
                LogStatusEnum::Warning
            } else {
                LogStatusEnum::Error
            },
            status_code: Some(status),
            http_method,
            endpoint: format!("{:?}", endpoint),
            request_header: format!("{:?}", request_header),
            request_body: None,
            request_params: None,
            response_body: body,
            process_taken_time: Some(response_time),
        };

        log.write_to_file()
    }

    pub fn error(
        http_method: String,
        endpoint: String,
        request_header: String,
        error_message: String,
        response_time: u128,
    ) -> io::Result<()> {
        let log = Self {
            current_time: Instant::now(),
            log_status: LogStatusEnum::Error,
            status_code: Some(StatusCode::INTERNAL_SERVER_ERROR),
            http_method,
            endpoint: format!("{:?}", endpoint),
            request_header: format!("{:?}", request_header),
            request_body: None,
            request_params: None,
            response_body: Some(error_message),
            process_taken_time: Some(response_time),
        };

        log.write_to_file()
    }

    fn write_to_file(&self) -> io::Result<()> {
        let date_str = Utc::now().with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap()).format("%Y-%m-%d");
        let filename = format!("{}_apps.log", date_str);
        let log_file = PathBuf::from(filename);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;

        let current_time = Utc::now().with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap()).format("%Y-%m-%dT%H:%M:%S%.3f%:z");

        writeln!(
            file,
            "{}|{:?}|{:?}|{}|{}|{:?}|{:?}|{:?}|{:?}|{:?}",
            current_time,
            self.log_status,
            self.status_code.map_or("".to_string(), |code| code.as_u16().to_string()),
            self.http_method,
            self.endpoint,
            self.request_header,
            self.request_body.as_deref().unwrap_or(""),
            self.request_params.as_deref().unwrap_or(""),
            self.response_body.as_deref().unwrap_or(""),
            self.process_taken_time.map_or("".to_string(), |code| code.to_string()),
        )?;

        Ok(())
    }
}