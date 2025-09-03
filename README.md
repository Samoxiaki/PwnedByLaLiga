# PwnedByLaLiga
 
A tiny Rust HTTP service to check if your server is still reachable **through Cloudflare**.  
Why? Javier Tebas.

---

## How it works
- Runs on a port you choose.
- Exposes a single, secret endpoint you define.
- Returns `OK` on that path, `404` everywhere else.

---

## Usage
```bash
pwnbll <port> <endpoint>
```

---

## Test with curl
```bash
curl -i http://127.0.0.1:9669/abcdefghijklmnopqrstuvwxyz
```

---

## Systemd example unit
```ini
[Unit]
Description=pwnbll - Cloudflare sanity check
After=network.target

[Service]
ExecStart=/usr/bin/pwnbll 9669 abcdefghijklmnopqrstuvwxyz
Restart=always
User=user

[Install]
WantedBy=multi-user.target
```