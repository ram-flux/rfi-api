//
//  Copyright 2024 Ram Flux, LLC.
//

mod parameter;

use axum::{Extension, Json};
use axum_extra::extract::WithRejection;

pub async fn init(
    Extension(app): Extension<models::ArcLockAppState>,
    // axum_client_ip::SecureClientIp(client_ip): axum_client_ip::SecureClientIp,
    WithRejection(Json(from), _): WithRejection<Json<parameter::DeviceInit>, common::ApiError>,
) -> Result<common::Response<parameter::DeviceRes>, common::ApiError> {
    let app = app.read().await;
    let db = &app.db;
    // let rd = &app.rd;

    let device_proof = from.uuid.clone();
    let account_hex = from.account.clone();
    let signature_hex = from.signature.clone();

    // println!("from: {:#?}", from);

    let account_pubkey = common::secp256k::from_pubkey_hex(account_hex.as_str())
        .map_err(|e| {
            common::ApiError::InternalServerError(format!("Failed to parse pubkey: {}", e))
        })
        .unwrap();

    let signature = common::secp256k::from_sig_hex(signature_hex.as_str()).unwrap();

    println!(
        "device_uuid.as_bytes.len: {:#?}",
        device_proof.as_bytes().len()
    );

    let verify =
        common::secp256k::verify_signature(device_proof.as_bytes(), &signature, &account_pubkey)
            .unwrap();

    if !verify {
        return Err(common::ApiError::Msg(
            "Failed to verify signature".to_string(),
        ));
    }

    let by_acc = models::Account::get_by_pubkey(db, account_hex.as_str())
        .await
        .unwrap();

    if !by_acc.is_some() {
        models::Account::create(
            db,
            &models::Account {
                id: 0,
                account: from.account.clone(),
                created_at: Some(models::fun::get_current_date()),
                updated_at: Some(models::fun::get_current_date()),
            },
        )
        .await
        .unwrap();
    }

    let by_proof = models::AccountDevice::get_by_proof(db, device_proof.as_str())
        .await
        .unwrap();

    let server_osrng = common::server_osrng().unwrap();

    let (server_prikey, server_pubkey) =
        common::server_generate(device_proof.as_bytes(), &account_pubkey.serialize()).unwrap();

    if by_proof.is_some() {
        let resdb = models::AccountDevice::init_update(
            db,
            &&server_pubkey.clone(),
            &server_prikey.clone(),
            &server_osrng.clone(),
            &models::fun::get_current_date(),
            &device_proof.clone(),
            &from.account.clone(),
        )
        .await
        .unwrap();
        println!("resdb: {:#?}", resdb);
    } else {
        models::AccountDevice::create(
            db,
            &models::AccountDevice {
                id: 0,
                account: Some(from.account.clone()),
                public_key: Some("".to_string()),
                prikey: Some(server_prikey.clone()),
                pubkey: Some(server_pubkey.clone()),
                proof: Some(device_proof.clone()),
                osrng: Some(server_osrng.clone()),
                def: Some("".to_string()),
                token: Some("".to_string()),
                versions: Some("".to_string()),
                ext: Some("".to_string()),
                last_ip: Some("".to_string()),
                status: Some(1),
                last_time: Some(models::fun::get_current_date()),
                created_at: Some(models::fun::get_current_date()),
                updated_at: Some(models::fun::get_current_date()),
            },
        )
        .await
        .unwrap();
    }

    Ok(parameter::DeviceRes {
        osrng: server_osrng,
        pubkey: server_pubkey,
    }
    .into())
}

pub async fn binding(
    Extension(app): Extension<models::ArcLockAppState>,
    payload: String,
) -> Result<common::Response<String>, common::ApiError> {
    let app = app.read().await;
    let db = &app.db;
    // let rd = &app.rd;
    if payload.is_empty() && payload.len() == 0 {
        return Err(common::ApiError::Msg("payload is empty".to_string()));
    }

    let parts: Vec<String> = payload.split('.').map(|part| part.to_string()).collect();

    if parts.len() != 3 {
        return Err(common::ApiError::Msg("payload err".to_string()));
    }

    let data = parts[0].clone();
    let osrng = parts[1].clone();
    let pubkey = parts[2].clone();

    let device = models::AccountDevice::get_by_pubkey_osrng(db, pubkey.as_str(), osrng.as_str())
        .await
        .unwrap();

    if let Some(device) = device {
        let proof = device.proof.unwrap();
        let account = device.account.unwrap();
        let ser_pri = device.prikey.unwrap();

        let (_, dev_pub) = common::x25519Device::generate_device_key(
            proof.as_bytes(),
            account.as_bytes(),
            device.osrng.unwrap(),
        )?;

        let json_str = common::Encrypt::new(
            ser_pri,
            hex::encode(dev_pub.as_bytes()),
            "unique nonce".to_string(),
            data,
        )
        .decrypt()
        .unwrap();
        let device = serde_json::from_str::<parameter::DeviceBinding>(json_str.as_str()).unwrap();

        if device.device_pubkey.is_some() {
            let device_pubkey = device.device_pubkey.unwrap();
            let binding_update = models::AccountDevice::binding_update(
                db,
                &device_pubkey,
                &models::fun::get_current_date(),
                &proof,
                &account,
            )
            .await
            .unwrap();
            println!("binding_update: {:#?}", binding_update);
            return Ok("OK".to_string().into());
        }

        return Err(common::ApiError::Msg("data err".to_string()));
    }
    return Err(common::ApiError::Msg("data err".to_string()));
}

pub async fn update(
    Extension(app): Extension<models::ArcLockAppState>,
    WithRejection(Json(from), _): WithRejection<Json<parameter::DeviceReq>, common::ApiError>,
) -> Result<common::Response<String>, common::ApiError> {
    let app = app.read().await;
    let db = &app.db;
    // println!("from: {:#?}", from);
    let payload = from.data;
    let signature = from.signature;

    if payload.is_empty() && payload.len() == 0 {
        return Err(common::ApiError::Msg("payload is empty".to_string()));
    }

    let parts: Vec<String> = payload.split('.').map(|part| part.to_string()).collect();

    if parts.len() != 2 {
        return Err(common::ApiError::Msg("payload err".to_string()));
    }

    let data = parts[0].clone();
    let device_pubkey = parts[1].clone();

    let device = models::AccountDevice::get_by_device_pubkey(db, device_pubkey.clone().as_str())
        .await
        .unwrap();

    if let Some(device) = device {
        // println!("device: {:#?}", device);

        let proof = device.proof.unwrap();
        let ser_pri = device.prikey.unwrap();

        // println!("serpubk:{}", device.pubkey.unwrap());

        let sig = common::signature::Signature::new(ser_pri.clone(), device_pubkey.clone())
            .verify(data.as_str(), signature.as_str())?;
        // println!("sig is: {:#?}", sig);

        if !sig {
            return Err(common::ApiError::Msg("signature err".to_string()));
        }

        let iproof = common::Encrypt::new(
            ser_pri,
            device_pubkey.clone(),
            "unique nonce".to_string(),
            data,
        )
        .decrypt()
        .unwrap();

        if iproof == proof {
            let account_pubkey_hex = device.account.unwrap();
            let account_pubkey = common::secp256k::from_pubkey_hex(account_pubkey_hex.as_str())
                .map_err(|e| {
                    common::ApiError::InternalServerError(format!("Failed to parse pubkey: {}", e))
                })
                .unwrap();

            let (server_prikey, server_pubkey) =
                common::server_generate(proof.as_bytes(), &account_pubkey.serialize()).unwrap();

            let bl = models::AccountDevice::update_ser_pri_pub(
                db,
                device_pubkey.as_str(),
                &models::fun::get_current_date(),
                server_pubkey.clone().as_str(),
                server_prikey.as_str(),
            )
            .await
            .unwrap();

            if bl {
                return Ok(server_pubkey.into());
            } else {
                return Err(common::ApiError::Msg("db err".to_string()));
            }
        } else {
            return Err(common::ApiError::Msg("proof err".to_string()));
        }
    }
    return Err(common::ApiError::Msg("data err".to_string()));
}

pub async fn delete(
    Extension(app): Extension<models::ArcLockAppState>,
    WithRejection(Json(from), _): WithRejection<Json<parameter::DeviceReq>, common::ApiError>,
) -> Result<common::Response<String>, common::ApiError> {
    let app = app.read().await;
    let db = &app.db;
    println!("from: {:#?}", from);
    let payload = from.data;
    let signature = from.signature;

    if payload.is_empty() && payload.len() == 0 {
        return Err(common::ApiError::Msg("payload is empty".to_string()));
    }

    let parts: Vec<String> = payload.split('.').map(|part| part.to_string()).collect();

    if parts.len() != 2 {
        return Err(common::ApiError::Msg("payload err".to_string()));
    }

    let data = parts[0].clone();
    let device_pubkey = parts[1].clone();

    let device = models::AccountDevice::get_by_device_pubkey(db, device_pubkey.clone().as_str())
        .await
        .unwrap();

    if let Some(device) = device {
        // println!("device: {:#?}", device);

        let proof = device.proof.unwrap();
        let ser_pri = device.prikey.unwrap();

        println!("serpubk:{}", device.pubkey.unwrap());

        let sig = common::signature::Signature::new(ser_pri.clone(), device_pubkey.clone())
            .verify(data.as_str(), signature.as_str())?;
        println!("sig is: {:#?}", sig);

        if !sig {
            return Err(common::ApiError::Msg("signature err".to_string()));
        }

        let iproof = common::Encrypt::new(
            ser_pri,
            device_pubkey.clone(),
            "unique nonce".to_string(),
            data,
        )
        .decrypt()
        .unwrap();

        if iproof == proof {
            let bl = models::AccountDevice::delete_by_pubkey(db, device_pubkey.as_str())
                .await
                .unwrap();
            if bl {
                return Ok("OK".to_string().into());
            } else {
                return Err(common::ApiError::Msg("db err".to_string()));
            }
        } else {
            return Err(common::ApiError::Msg("proof err".to_string()));
        }
    }
    return Err(common::ApiError::Msg("data err".to_string()));
}
