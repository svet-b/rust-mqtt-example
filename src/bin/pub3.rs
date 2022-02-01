use mqttexample::mqtt_publish;
use std::thread;

fn main() {
    thread::spawn(|| {
        mqtt_publish(5);
    });

    mqtt_publish(10);
}
 