
use std::sync::Arc;
use axum::{
    extract:: State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::ZkpModel,
    schema::ZkpSignUpSchema,
    AppState,
    zkpgenerate::{zkpproof_sign_in, zkpproof_sign_up},
};

use crate::sol_connect::{user_sign_up, user_sign_in};
pub async fn zkp_signup(
    State(data): State<Arc<AppState>>,
    Json(body): Json<ZkpSignUpSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let embeddinghash_num = hash_to_array(&body.embedding_hash);
    let embeddinghash_num_arr = embeddinghash_num.unwrap();
    let microchipid_num_arr = body.microchip_id.to_be_bytes();
    let public_input = zkpproof_sign_up(embeddinghash_num_arr, microchipid_num_arr);
    // let public_input_txt = vec![public_input[0].to_string(), public_input[1].to_string()];
    // println!("public_input : {:?}", public_input[0]);
    // let query_result = sqlx::query_as::<_, ZkpModel>(r#"INSERT INTO zkptable (dog_id, public_input) VALUES ($1, $2) RETURNING *"#)
    // .bind(body.dog_id)
    // .bind(&public_input_txt)
    // .fetch_one(&data.db)
    // .await;
    // match query_result {
    //     Ok(zkp_data) => {
    //         let zkp_data_response = json!({"status": "success","data": json!({
    //             "zkp_data": zkp_data
    //         })});

    //         return Ok((StatusCode::CREATED, Json(zkp_data_response)));
    //     }
    //     Err(e) => {
    //         if e.to_string()
    //             .contains("duplicate key value violates unique constraint")
    //         {
    //             let error_response = serde_json::json!({
    //                 "status": "fail",
    //                 "message": "Public Input with that title already exists",
    //             });
    //             return Err((StatusCode::CONFLICT, Json(error_response)));
    //         }
    //         return Err((
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             Json(json!({"status": "error","message": format!("{:?}", e)})),
    //         ));
    //     }
    // }
    user_sign_up(public_input);
    Ok({})
}

pub async fn zkp_signin(
    State(data): State<Arc<AppState>>,
    Json(body): Json<ZkpSignUpSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let embeddinghash_num = hash_to_array(&body.embedding_hash);
    let embeddinghash_num_arr = embeddinghash_num.unwrap();
    let microchipid_num_arr = body.microchip_id.to_be_bytes();
    // let public_input_txt = sqlx::query_as::<_, ZkpModel>(r#"SELECT * FROM zkptable WHERE dog_id = $1"#)
    // .bind(body.dog_id)
    // .fetch_one(&data.db)
    // .await;
    // println!("Zkp row : {:?}", public_input_txt);
    // if public_input_txt.is_err() {
    //     let error_response = serde_json::json!({
    //         "status": "fail",
    //         "message": format!("Public Input with ID: {} not found", body.dog_id)
    //     });
    //     return Err((StatusCode::NOT_FOUND, Json(error_response)));
    // }
    let proof = zkpproof_sign_in(embeddinghash_num_arr, microchipid_num_arr);
    user_sign_in(proof);

    return Ok({});
}

fn hash_to_array(hash: &str) -> Result<[u8; 64], String> {
    if hash.len() != 64 {
        return Err(format!("Expected 64 characters, got {}", hash.len()));
    }

    let mut result = [0u8; 64];
    for (i, chunk) in hash.as_bytes().chunks(2).enumerate() {
        let hash_str = std::str::from_utf8(chunk).map_err(|_| "Invalid UTF-8 in hash")?;
        result[i] = u8::from_str_radix(hash_str, 16).map_err(|_| "Invalid hash digit")?;
    }
    Ok(result)
}
