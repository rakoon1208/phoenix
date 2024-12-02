use madgwick::{F32x3};
use madgwick_lib::{MadgwickFilter, mock_sensor_data};

#[test]
fn test_madgwick_filter() {
    let mut filter = MadgwickFilter::new(0.1, 100.0);

    // Mock data
    let (gyro, accel) = mock_sensor_data();

    // Update the filter
    let orientation = filter.update_imu(gyro, accel);

    // Verify the result
    println!("Orientation: {:?}", orientation);
    assert!(orientation.x.abs() <= 1.0);
    assert!(orientation.y.abs() <= 1.0);
    assert!(orientation.z.abs() <= 1.0);
    assert!(orientation.w.abs() <= 1.0);
}
