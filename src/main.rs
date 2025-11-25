fn main() {
    println!("Hello, world knucklehead!");
}


/*

5. Run the App Automatically on Startup

For a command-line app, the simplest method is a systemd service.

Create a service file:

sudo nano /etc/systemd/system/rust_pi_timer.service


Paste:

[Unit]
Description=Rust Pi Timer App
After=network.target

[Service]
ExecStart=/home/pi/rust_pi_timer/target/release/rust_pi_timer
WorkingDirectory=/home/pi/rust_pi_timer
Restart=always
User=pi

[Install]
WantedBy=multi-user.target


Enable and start it:

sudo systemctl daemon-reload
sudo systemctl enable rust_pi_timer.service
sudo systemctl start rust_pi_timer.service


Check status/logs:

sudo systemctl status rust_pi_timer.service
journalctl -u rust_pi_timer.service -f


*/