use madgwick::{Madgwick, F32x3};

pub struct MadgwickFilter {
    filter: Madgwick,
}

impl MadgwickFilter {
    pub fn new(beta: f32, frequency: f32) -> Self {
        Self {
            filter: Madgwick::new(beta, frequency),
        }
    }

    // Updates the filter with gyroscope and accelerometer data
    pub fn update_imu(&mut self, gyro: F32x3, accel: F32x3) -> F32x3 {
        self.filter.update_imu(gyro, accel);
        self.filter.quaternion()
    }
}

pub fn mock_sensor_data() -> (F32x3, F32x3) {
    // Simulate gyroscope and accelerometer readings
    let gyro = F32x3::new(0.01, 0.02, 0.03);   // gyroscope data (rad/s)
    let accel = F32x3::new(0.0, 0.0, -1.0);    // accelerometer data (m/s^2)
    (gyro, accel)
}

