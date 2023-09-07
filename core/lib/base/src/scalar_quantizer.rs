use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
struct ProductQuantizerSettings{
    pub updated: bool,
    pub offsets:Vec<f32>,
    pub alphas: Vec<f32>,

}

#[derive(Debug)]
pub struct ProductQuantizer {
    quantile: f32,
    settings: Arc<RwLock<ProductQuantizerSettings>>,
    pre_compute_by_vid: Arc<RwLock<HashMap<usize, f32>>>,
}


impl ProductQuantizer {
    fn gen_quantize_params(&self, arr_a: &[f32], subspace_size: usize) -> (Vec<f32>, Vec<f32>) {
        let mut offsets = Vec::new();
        let mut alphas = Vec::new();
        let num_subspaces = arr_a.len() / subspace_size;

        for i in 0..num_subspaces {
            let subspace_data = &arr_a[i * subspace_size..(i + 1) * subspace_size];

            // Compute offset and alpha for each subspace.
            // You can replace this with more sophisticated logic if necessary.
            let offset = *subspace_data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let alpha = *subspace_data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() - offset;

            offsets.push(offset);
            alphas.push(alpha);
        }

        // Store these new offsets and alphas into the settings
        let mut settings_w = self.settings.write();
        settings_w.offsets = offsets.clone();
        settings_w.alphas = alphas.clone();
        settings_w.updated = true;

        (offsets, alphas)
    }

    pub fn quantize_arr(&self, arr_a: &[f32], subspace_size: usize) -> Vec<u8> {
        let settings_r = self.settings.read();
        let offsets = &settings_r.offsets;
        let alphas = &settings_r.alphas;

        let mut quantized_data = Vec::new();

        let num_subspaces = arr_a.len() / subspace_size;
        for i in 0..num_subspaces {
            let subspace_data = &arr_a[i * subspace_size..(i + 1) * subspace_size];
            let offset = offsets[i];
            let alpha = alphas[i];
            let quantized_subspace: Vec<u8> = subspace_data
                .iter()
                .map(|x| ((x - offset) / alpha) as u8)
                .collect();
            quantized_data.extend_from_slice(&quantized_subspace);
        }

        quantized_data
    }

    pub fn new(quantile: f32) -> anyhow::Result<ProductQuantizer> {
        Ok(ProductQuantizer {
            quantile,
            settings: Arc::new(RwLock::new(ProductQuantizerSettings {
                updated: false,
                offsets: vec![],
                alphas: vec![],
            })),
            pre_compute_by_vid: Arc::new(RwLock::new(HashMap::new())),
        })
    }
}
