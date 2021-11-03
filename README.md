# nespi-rs
Safe shutdown utils for NESPi Case+ written in Rust.

## usage
Intended to be used with Raspberry Pi OS Lite.

## installation
Download latest release to your Raspberry Pi:

```
wget https://github.com/zduny/nespi-rs/releases/download/v0.1.0/nespi-rs-0.1.0.zip
```

Unzip the archive:
```
unzip nespi-rs-0.1.0.zip 
```

Move `safe_shutdown`:
```
sudo mv safe_shutdown /usr/local/bin/safe_shutdown 
```

Move `safe_shutdown.service`:
```
sudo mv safe_shutdown.service /etc/systemd/system/safe_shutdown.service 
```

Enable service:
```
sudo systemctl daemon-reload
sudo systemctl enable safe_shutdown
```

Move `power_down`:
```
sudo mv power_down /usr/lib/systemd/system-shutdown/power_down
```

## disclaimer
This project is in no way affiliated with the Retroflag brand/company.

## see also
[Retroflag](https://retroflag.com/)
