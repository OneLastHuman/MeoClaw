//! OpenClaw 设备身份认证
//!
//! 实现 Ed25519 challenge-response 认证流程

use base64::{engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signer, SigningKey};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

/// 设备身份信息
#[derive(Debug, Clone)]
pub struct DeviceIdentity {
    pub device_id: String,
    pub public_key: Vec<u8>,
    signing_key: SigningKey,
}

impl DeviceIdentity {
    /// 从 OpenClaw identity 文件加载
    pub fn load(openclaw_dir: Option<PathBuf>) -> Result<Self, String> {
        let dir = openclaw_dir.unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_default()
                .join(".openclaw")
        });

        let identity_path = dir.join("identity").join("device.json");
        let paired_path = dir.join("devices").join("paired.json");

        let identity_data = std::fs::read_to_string(&identity_path)
            .map_err(|e| format!("Failed to read {}: {}", identity_path.display(), e))?;

        let device: DeviceJson = serde_json::from_str(&identity_data)
            .map_err(|e| format!("Failed to parse device.json: {}", e))?;

        // 从 PEM 解析 Ed25519 私钥
        let private_key_pem = device.private_key_pem;
        let signing_key = parse_ed25519_private_key(&private_key_pem)
            .map_err(|e| format!("Failed to parse private key: {}", e))?;

        // 计算公钥
        let public_key = signing_key.verifying_key().to_bytes().to_vec();

        // 计算 device_id (SHA256 of public key, hex encoded)
        let device_id = {
            let mut hasher = Sha256::new();
            hasher.update(&public_key);
            hex::encode(hasher.finalize())
        };

        // 获取 gateway token (使用当前设备的 token)
        let paired_data = std::fs::read_to_string(&paired_path)
            .map_err(|e| format!("Failed to read {}: {}", paired_path.display(), e))?;

        let paired: PairedDevices = serde_json::from_str(&paired_data)
            .map_err(|e| format!("Failed to parse paired.json: {}", e))?;

        // 找到当前设备的 operator token
        let token = find_operator_token_for_device(&paired, &device_id)
            .ok_or_else(|| "No operator token found for this device".to_string())?;

        log::info!("[Auth] Loaded device identity: {}", &device_id[..16]);
        log::info!("[Auth] Gateway token: {}...", &token[..8]);

        Ok(Self {
            device_id,
            public_key,
            signing_key,
        })
    }

    /// 构建 v2 签名 payload（官方 CLI 使用的格式）
    pub fn build_v2_payload(
        &self,
        client_id: &str,
        client_mode: &str,
        role: &str,
        scopes: &[String],
        token: &str,
        nonce: &str,
        signed_at_ms: i64,
    ) -> String {
        format!(
            "v2|{}|{}|{}|{}|{}|{}|{}|{}",
            self.device_id,
            client_id,
            client_mode,
            role,
            scopes.join(","),
            signed_at_ms,
            token,
            nonce,
        )
    }

    /// 构建 v3 签名 payload
    pub fn build_v3_payload(
        &self,
        client_id: &str,
        client_mode: &str,
        role: &str,
        scopes: &[String],
        token: &str,
        nonce: &str,
        platform: &str,
        device_family: &str,
        signed_at_ms: i64,
    ) -> String {
        let payload = format!(
            "v3|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.device_id,
            client_id,
            client_mode,
            role,
            scopes.join(","),
            signed_at_ms,
            token,
            nonce,
            platform,
            device_family,
        );
        log::info!("[Auth] V3 payload: {}", payload);
        payload
    }

    /// 获取公钥 base64url 编码
    pub fn public_key_b64(&self) -> String {
        URL_SAFE_NO_PAD.encode(&self.public_key)
    }

    /// 对 challenge 签名
    pub fn sign(&self, payload: &[u8]) -> Vec<u8> {
        let signature = self.signing_key.sign(payload);
        signature.to_bytes().to_vec()
    }

    /// 加载 gateway token（从 paired.json）
    pub fn load_gateway_token(device_id: &str) -> Result<String, String> {
        let paired_path = dirs::home_dir()
            .ok_or_else(|| "Cannot find home directory".to_string())?
            .join(".openclaw")
            .join("devices")
            .join("paired.json");

        let paired_data = std::fs::read_to_string(&paired_path)
            .map_err(|e| format!("Failed to read {}: {}", paired_path.display(), e))?;

        let paired: PairedDevices = serde_json::from_str(&paired_data)
            .map_err(|e| format!("Failed to parse paired.json: {}", e))?;

        find_operator_token_for_device(&paired, device_id)
            .ok_or_else(|| "No operator token found".to_string())
    }
}

/// Paired devices JSON 结构
#[derive(Debug, Deserialize)]
struct PairedDevices {
    #[serde(flatten)]
    devices: std::collections::HashMap<String, PairedDevice>,
}

#[derive(Debug, Deserialize)]
struct PairedDevice {
    #[serde(rename = "deviceId")]
    device_id: String,
    tokens: Option<DeviceTokens>,
}

#[derive(Debug, Deserialize)]
struct DeviceTokens {
    operator: Option<TokenInfo>,
}

#[derive(Debug, Deserialize)]
struct TokenInfo {
    token: String,
}

#[derive(Debug, Deserialize)]
struct DeviceJson {
    #[serde(rename = "deviceId")]
    device_id: String,
    #[serde(rename = "privateKeyPem")]
    private_key_pem: String,
}

/// 找到指定设备的 operator token
fn find_operator_token_for_device(paired: &PairedDevices, device_id: &str) -> Option<String> {
    // 先尝试找当前设备
    if let Some(device) = paired.devices.get(device_id) {
        if let Some(tokens) = &device.tokens {
            if let Some(op) = &tokens.operator {
                return Some(op.token.clone());
            }
        }
    }
    // 如果没找到，尝试在任何设备中找
    for device in paired.devices.values() {
        if device.device_id == device_id {
            if let Some(tokens) = &device.tokens {
                if let Some(op) = &tokens.operator {
                    return Some(op.token.clone());
                }
            }
        }
    }
    None
}

/// 解析 Ed25519 PEM 私钥
fn parse_ed25519_private_key(pem: &str) -> Result<SigningKey, String> {
    // 移除 PEM 头尾和空白
    let pem = pem.trim();
    // 处理 JSON 中的字面 \n 字符串，替换为真正换行符
    let pem = pem.replace("\\n", "\n");
    let base64_data = pem
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<Vec<_>>()
        .join("");

    // 解码 base64 (PEM 使用标准 base64)
    let key_bytes = STANDARD
        .decode(&base64_data)
        .map_err(|e| format!("Base64 decode error: {}", e))?;

    // Ed25519 私钥可能是原始 32 字节格式或 PKCS8 格式 (48 字节)
    // PKCS8 格式的最后 32 字节就是 Ed25519 seed
    let seed: [u8; 32] = if key_bytes.len() == 32 {
        key_bytes.try_into().unwrap()
    } else if key_bytes.len() == 48 {
        // PKCS8 格式，seed 在字节 16-47 (32 字节)
        key_bytes[16..48].try_into().unwrap()
    } else {
        return Err(format!(
            "Expected 32 or 48 byte Ed25519 key, got {}",
            key_bytes.len()
        ));
    };

    Ok(SigningKey::from_bytes(&seed))
}