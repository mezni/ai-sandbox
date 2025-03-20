pub struct RoamOutRecord {
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
}

pub struct EnrichedRoamOutRecord {
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
    pub carrier_id: String,
    pub carrier_name: String,
    pub country_name: String,
}

pub trait RoamOutRecordRepository {
    fn get_roam_out_records(&self) -> Vec<RoamOutRecord>;
}

pub trait EnrichedRoamOutRecordRepository {
    fn save_enriched_record(&self, record: EnrichedRoamOutRecord);
}