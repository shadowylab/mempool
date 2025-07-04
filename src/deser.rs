pub(crate) mod weight_serde {
    use bitcoin::Weight;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(weight: &Weight, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(weight.to_vbytes_ceil())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Weight, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vbytes = u64::deserialize(deserializer)?;
        Ok(Weight::from_vb_unchecked(vbytes))
    }
}

pub(crate) mod fee_rate_u64_serde {
    use bitcoin::FeeRate;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(fee_rate: &FeeRate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let sat_per_vb: u64 = fee_rate.to_sat_per_vb_ceil();
        serializer.serialize_u64(sat_per_vb)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<FeeRate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let sat_per_vb = u64::deserialize(deserializer)?;
        Ok(FeeRate::from_sat_per_vb_unchecked(sat_per_vb))
    }
}

pub(crate) mod fee_rate_f64_serde {
    use bitcoin::FeeRate;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(fee_rate: &FeeRate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert from sat/kwu to sat/vB
        let sat_per_vb: f64 = fee_rate.to_sat_per_vb_ceil() as f64;
        serializer.serialize_f64(sat_per_vb)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<FeeRate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let sat_per_vb: f64 = f64::deserialize(deserializer)?;
        Ok(FeeRate::from_sat_per_vb_unchecked(sat_per_vb as u64))
    }
}

pub(crate) mod fee_rate_vec_serde {
    use bitcoin::FeeRate;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(fee_rate: &[FeeRate], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fee_rates_f64: Vec<f64> = fee_rate
            .iter()
            .map(|fr| fr.to_sat_per_kwu() as f64 / 1000.0)
            .collect();

        fee_rates_f64.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<FeeRate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let fee_rates_f64: Vec<f64> = Vec::deserialize(deserializer)?;

        let fee_rates = fee_rates_f64
            .into_iter()
            .map(|sat_per_vb| {
                let sat_per_kwu: u64 = (sat_per_vb * 1000.0) as u64;
                FeeRate::from_sat_per_kwu(sat_per_kwu)
            })
            .collect();

        Ok(fee_rates)
    }
}

pub(crate) mod fee_histogram_serde {
    use bitcoin::{FeeRate, Weight};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::response::FeeHistogramEntry;

    pub fn serialize<S>(histogram: &[FeeHistogramEntry], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let list: Vec<(f64, u64)> = histogram
            .iter()
            .map(|entry| {
                (
                    entry.fee_rate.to_sat_per_kwu() as f64 / 1000.0,
                    entry.vsize.to_vbytes_ceil(),
                )
            })
            .collect();

        list.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<FeeHistogramEntry>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let array_format: Vec<[f64; 2]> = Vec::deserialize(deserializer)?;

        let histogram = array_format
            .into_iter()
            .map(|[fee_rate_sat_per_vb, vsize]| FeeHistogramEntry {
                fee_rate: FeeRate::from_sat_per_kwu((fee_rate_sat_per_vb * 1000.0) as u64),
                vsize: Weight::from_vb_unchecked(vsize as u64),
            })
            .collect();

        Ok(histogram)
    }
}
