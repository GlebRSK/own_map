use crate::models::{Status, GeoPoint}; 
use crate::db;

use std::io;
use actix_web::{Responder, HttpResponse, web};
use deadpool_postgres::{Pool, Client};


