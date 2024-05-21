## 1 base64 编码和解码

### 1.1 encode

```
cargo run -- base64 encode -i Cargo.toml --format urlsafe >tmp.bs
```

### 1.2 decode

```
argo run -- base64 decode --format urlsafe -i tmp.bs > tmp1.bs
```

## 2 Text 签名和验证

### 2.1 generate 生成密钥

```
// 生成blake3类型的密钥
cargo run -- text generate -o fixtures

// 生成ed25519类型的密钥（公钥和私钥）
cargo run -- text generate -o fixtures --format ed25519
```

### 2.2 sign 签名

```
// blake3
cargo run -- text sign -k fixtures/blake3.txt

输入:hello  两个control+d

hello:33Ypo4rveYpWmJKAiGnnse-wHQhMVujjmcVkV4Tl43k

// ed25519
cargo run -- text sign -k fixtures/ed25519.sk --format=ed25519
hello:-Zp4PoDFKOLn5hntITFBWOoLTRySzxnZbUHbQntUvwMRnSdB1EeA_wwo_0sfBisMPksb7RB8QBeh3GCS1op-Dw
```

### 2.3 verify 验证

```
// blake3
cargo run -- text verify -k fixtures/blake3.txt --sig xxxxxxxxxxxxxxxxxxx --format blake3
```

```
// ed25519
cargo run -- text verify -k fixtures/ed25519.pk --sig uRfLOdUmz0Ro1tOvP_e-mxdmk6B9-9Ak1P_iWTQN0Ieigt5Igdzi2MPTk7LPUk60Z_zoC6PukIlZB6kkq_snDw --format ed25519

```

## 3 测试

```
cargo nextest run -- test_ed25519_sign_verify

cargo test
```
