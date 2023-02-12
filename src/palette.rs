use std::{cmp::Ordering, ops::Deref};
use crate::color;

struct Histogram;

type HistogramBins = [[f32; 3]; Histogram::HISTOGRAM_SIZE];
type HistogramWeights = [f32; Histogram::HISTOGRAM_SIZE];

type HistogramResult = (HistogramBins, HistogramWeights);

impl Histogram {
    const HISTOGRAM_SIZE: usize = 4096;

    fn compute<RgbImageData>(image_data: &RgbImageData) -> HistogramResult
    where
        RgbImageData: Deref<Target = [u8]>,
    {
        let mut bins: [[f32; 3]; Histogram::HISTOGRAM_SIZE] =
            [[0., 0., 0.]; Histogram::HISTOGRAM_SIZE];
        let mut weights: [f32; Histogram::HISTOGRAM_SIZE] = [0.0; Histogram::HISTOGRAM_SIZE];

        image_data
            .chunks_exact(3)
            .map(|rgb| {
                let rgb = rgb.try_into().unwrap();
                let lab = color::rgb2lab(rgb);
                let bin_index: usize = (((rgb[0] as f32 / 16.0).floor() * 16.0
                    + (rgb[1] as f32 / 16.0).floor())
                    * 16.0
                    + (rgb[2] as f32 / 16.0).floor())
                .floor() as usize;

                (lab, bin_index)
            })
            .for_each(|(lab, index)| {
                weights[index] += 1.0;

                bins[index][0] = bins[index][0] + lab[0];
                bins[index][1] = bins[index][1] + lab[1];
                bins[index][2] = bins[index][2] + lab[2];
            });

        (bins, weights)
    }
}

fn euclidian_distance_vec3(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    let x = a[0] - b[0];
    let y = a[1] - b[1];
    let z = a[2] - b[2];

    x * x + y * y + z * z
}

fn attenuate_weights(
    weights: &mut HistogramWeights,
    histogram: &HistogramResult,
    seed_color: &[f32; 3],
) {
    const SEPARATION_COEFFICIENT: f32 = 3650.0;

    histogram
        .1
        .iter()
        .enumerate()
        .filter(|(_, w)| **w > 0.0)
        .for_each(|(index, w)| {
            let target_color = [
                histogram.0[index][0] / *w,
                histogram.0[index][1] / *w,
                histogram.0[index][2] / *w,
            ];

            weights[index] = weights[index]
                * (1.0
                    - (-(euclidian_distance_vec3(seed_color, &target_color)
                        / SEPARATION_COEFFICIENT))
                        .exp());
        });
}

fn seeds_selection(palette_size: usize, hist_results: &HistogramResult) -> Vec<[f32; 3]> {
    let mut weights = hist_results.1.clone();

    let mut seeds: Vec<[f32; 3]> = Vec::with_capacity(palette_size);

    for _ in 0..palette_size {
        let (max_index, max_value): (usize, &f32) = weights
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| (a).partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap();

        if *max_value == 0.0 {
            break;
        }

        let seed = [
            hist_results.0[max_index][0] / hist_results.1[max_index] as f32,
            hist_results.0[max_index][1] / hist_results.1[max_index] as f32,
            hist_results.0[max_index][2] / hist_results.1[max_index] as f32,
        ];

        weights[max_index] = 0.0;
        seeds.push(seed);

        attenuate_weights(&mut weights, hist_results, &seed);
    }

    seeds
}

pub fn kmean_cluster_colors(
    seeds: &Vec<[f32; 3]>,
    hist_results: &HistogramResult,
) -> Vec<[f32; 3]> {
    let mut cluster_indices = [0; Histogram::HISTOGRAM_SIZE];
    let mut optimum = false;

    let mut clusters = vec![[0.0, 0.0, 0.0]; seeds.len()];

    while !optimum {
        optimum = true;

        let mut seed_weights = vec![0f32; seeds.len()];
        let mut new_seeds = vec![[0.0, 0.0, 0.0]; seeds.len()];

        hist_results
            .1
            .iter()
            .enumerate()
            .filter(|(_, w)| **w > 0.0)
            .for_each(|(index, w)| {
                let current_color = [
                    hist_results.0[index][0] / *w,
                    hist_results.0[index][1] / *w,
                    hist_results.0[index][2] / *w,
                ];

                let cluster_index = seeds
                    .iter()
                    .map(|color| euclidian_distance_vec3(color, &current_color))
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.total_cmp(b))
                    .map(|(index, _)| index)
                    .unwrap();

                if optimum && cluster_index != cluster_indices[index] {
                    optimum = false;
                }

                cluster_indices[index] = cluster_index;

                new_seeds[cluster_index] = [
                    new_seeds[cluster_index][0] + hist_results.0[index][0],
                    new_seeds[cluster_index][1] + hist_results.0[index][1],
                    new_seeds[cluster_index][2] + hist_results.0[index][2],
                ];

                seed_weights[cluster_index] += hist_results.1[index];
            });

        clusters
            .iter_mut()
            .enumerate()
            .filter(|(index, _)| seed_weights[*index] > 0.0)
            .for_each(|(index, value)| {
                *value = [
                    new_seeds[index][0] / seed_weights[index] as f32,
                    new_seeds[index][1] / seed_weights[index] as f32,
                    new_seeds[index][2] / seed_weights[index] as f32,
                ];
            });
    }

    clusters
}

fn convert_palette_to_rgb(palette: Vec<[f32; 3]>) -> Vec<[u8; 3]> {
    palette.iter().map(|x| color::lab2rgb(x)).collect()
}

pub fn extract_color_palette<RgbImageData>(image_data: &RgbImageData) -> Vec<[u8; 3]>
where
    RgbImageData: Deref<Target = [u8]>,
{
    let histogram = Histogram::compute(image_data);

    let seeds = seeds_selection(5, &histogram);
    let palette = kmean_cluster_colors(&seeds, &histogram);

    convert_palette_to_rgb(palette)
}