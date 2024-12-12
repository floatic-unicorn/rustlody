/* 지정된 파일의 locationCode 파싱용 파일 - 테스트 하기전에 넣어주기용 */
use std::{fs::File, io::BufReader};
use serde::Deserialize;

#[derive(Deserialize)]
struct BoxLocation {
    floor: i32,
    sequence: i32
}

#[derive(Deserialize)]
struct Sku {
    wmsSkuId: String,
    name: String,
    locationCode: String,
    skuBarcode: String
}

#[derive(Deserialize)]
struct Work {
    sku: Sku,
    requestQuantity: i32,
    isTamperEvident: bool
}

#[derive(Deserialize)]
struct WorkGroup {
    wmsWorkGroupId: String,
    trackingNumber: String,
    boxLocation: BoxLocation,
    works: Vec<Work>
}

#[derive(Deserialize)]
struct Wave {
    warehouseId: String,
    wmsWaveId: String,
    waveName: String,
    affinity: String,
    sellingChannel: String,
    boxType: String,
    maxBoxCount: i32,
    desiredBoxCount: i32,
    workGroups: Vec<WorkGroup>
}

pub fn parse_and_get_location_codes(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let waves: Vec<Wave> = serde_json::from_reader(reader).unwrap();

    let mut location_codes = vec![];
    for wave in waves {
        for work_group in wave.workGroups {
            for work in work_group.works {
                if !location_codes.contains(&work.sku.locationCode) {
                    location_codes.push(work.sku.locationCode);
                }
            }
        }
    }
    location_codes
}
