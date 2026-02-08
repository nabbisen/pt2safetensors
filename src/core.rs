use candle_core::pickle;
use safetensors::serialize_to_file;

use std::{collections::HashMap, fs, path::PathBuf};

pub struct Pt2Safetensors {
    removes_pt_at_conversion_success: bool,
}

impl Pt2Safetensors {
    pub fn removes_pt_at_conversion_success(mut self) -> Self {
        self.removes_pt_at_conversion_success = true;
        self
    }

    pub fn convert<T: Into<PathBuf>>(
        &self,
        pt_file_path: T,
        safetensors_file_path: T,
    ) -> Result<(), anyhow::Error> {
        let pt_file_path = pt_file_path.into();

        // 1. .binファイルを読み込む（内部でPickleパースとZip展開を行う）
        let weights = pickle::read_all(&pt_file_path)?;

        // 2. テンソルマップを構築
        let mut tensors = HashMap::new();
        for (name, tensor) in weights {
            tensors.insert(name, tensor);
        }

        // 3. .safetensorsとして保存
        serialize_to_file(&tensors, None, &safetensors_file_path.into())?;

        if self.removes_pt_at_conversion_success {
            fs::remove_file(&pt_file_path)?;
        }

        Ok(())
    }
}

impl Default for Pt2Safetensors {
    fn default() -> Self {
        Self {
            removes_pt_at_conversion_success: false,
        }
    }
}
