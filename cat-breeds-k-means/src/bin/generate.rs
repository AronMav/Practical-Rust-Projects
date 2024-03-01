use rand::thread_rng;
use rand::distributions::Distribution;
use rand_distr::Normal;
use ndarray::Array2;
use std::error::Error;
use std::io;

const CENTROIDS:[f64;6] = [
    //Height, length
    22.5, 40.5, // persian
    38.0, 50.0, // British shorthair
    25.5, 48.0, // Ragdoll
];

const NOISE:f64 = 1.8;
const SAMPLE_PER_CENTROID: usize = 2000;

fn generate_data(centroids: &Array2<f64>,
                 points_per_centroid: usize,
                 noise: f64)
                 -> Result<Array2<f64>, Box<dyn Error>> {
    assert!(!centroids.is_empty(), "centroids cannot be empty.");
    assert!(noise >= 0f64, "noise must be non-negative.");

    let rows = centroids.shape()[0];
    let cols = centroids.shape()[1];

    let mut rng = thread_rng();
    let normal_rv = Normal::new(0f64, noise)?; //[2]

    let mut raw_cluster_data = Vec::with_capacity(
        rows * points_per_centroid * cols);

    for _ in 0..points_per_centroid { //[3]
        //generate points from each centroid
        for centroid in centroids.rows() {
            //generate a point randomly around the centroid
            let mut point = Vec::with_capacity(
                centroids.shape()[1]
            );
            for feature in centroid.into_iter() {
                point.push(feature + normal_rv.sample(&mut rng));
            }

            //push point to raw_cluster_data
            raw_cluster_data.extend(point);
        }
    }

    Ok(Array2::from_shape_vec((rows * points_per_centroid, cols),
        raw_cluster_data)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let centroids = Array2::from_shape_vec(
        (3,2), CENTROIDS.to_vec()
    )?;
    
    let samples = generate_data(
        &centroids,
        SAMPLE_PER_CENTROID,
        NOISE
    )?;
    
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&["height", "length"])?;
    for sample in samples.rows() {
        let mut sample_iter = sample.into_iter();
        writer.serialize((
            sample_iter.next().unwrap(),
            sample_iter.next().unwrap()
            ))?;
    }
    Ok(())
}
