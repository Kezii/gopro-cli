use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use uuid::Uuid;

use tokio::time;

use log::info;

async fn find_gopro(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("GoPro"))
        {
            return Some(p);
        }
    }
    None
}

/*
Service UUID 00001801-0000-1000-8000-00805f9b34fb, primary: true
Service UUID 00001804-0000-1000-8000-00805f9b34fb, primary: true
  Characteristic { uuid: 00002a07-0000-1000-8000-00805f9b34fb, service_uuid: 00001804-0000-1000-8000-00805f9b34fb, properties: READ }
Service UUID 0000180a-0000-1000-8000-00805f9b34fb, primary: true
  Characteristic { uuid: 00002a23-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: READ }
  Characteristic { uuid: 00002a24-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: READ }
  Characteristic { uuid: 00002a25-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: READ }
  Characteristic { uuid: 00002a26-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: READ }
  Characteristic { uuid: 00002a27-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: READ }
  Characteristic { uuid: 00002a28-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: READ }
  Characteristic { uuid: 00002a29-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: READ }
  Characteristic { uuid: 00002a50-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: READ }
Service UUID 0000180f-0000-1000-8000-00805f9b34fb, primary: true
  Characteristic { uuid: 00002a19-0000-1000-8000-00805f9b34fb, service_uuid: 0000180f-0000-1000-8000-00805f9b34fb, properties: READ | NOTIFY }
Service UUID 0000fea6-0000-1000-8000-00805f9b34fb, primary: true
  Characteristic { uuid: b5f90072-aa8d-11e3-9046-0002a5d5c51b, service_uuid: 0000fea6-0000-1000-8000-00805f9b34fb, properties: WRITE }
  Characteristic { uuid: b5f90073-aa8d-11e3-9046-0002a5d5c51b, service_uuid: 0000fea6-0000-1000-8000-00805f9b34fb, properties: NOTIFY }
  Characteristic { uuid: b5f90074-aa8d-11e3-9046-0002a5d5c51b, service_uuid: 0000fea6-0000-1000-8000-00805f9b34fb, properties: WRITE }
  Characteristic { uuid: b5f90075-aa8d-11e3-9046-0002a5d5c51b, service_uuid: 0000fea6-0000-1000-8000-00805f9b34fb, properties: NOTIFY }
  Characteristic { uuid: b5f90076-aa8d-11e3-9046-0002a5d5c51b, service_uuid: 0000fea6-0000-1000-8000-00805f9b34fb, properties: WRITE }
  Characteristic { uuid: b5f90077-aa8d-11e3-9046-0002a5d5c51b, service_uuid: 0000fea6-0000-1000-8000-00805f9b34fb, properties: NOTIFY }
  Characteristic { uuid: b5f90078-aa8d-11e3-9046-0002a5d5c51b, service_uuid: 0000fea6-0000-1000-8000-00805f9b34fb, properties: WRITE }
  Characteristic { uuid: b5f90079-aa8d-11e3-9046-0002a5d5c51b, service_uuid: 0000fea6-0000-1000-8000-00805f9b34fb, properties: NOTIFY }
Service UUID b5f90001-aa8d-11e3-9046-0002a5d5c51b, primary: true
  Characteristic { uuid: b5f90002-aa8d-11e3-9046-0002a5d5c51b, service_uuid: b5f90001-aa8d-11e3-9046-0002a5d5c51b, properties: READ | WRITE }
  Characteristic { uuid: b5f90003-aa8d-11e3-9046-0002a5d5c51b, service_uuid: b5f90001-aa8d-11e3-9046-0002a5d5c51b, properties: READ | WRITE }
  Characteristic { uuid: b5f90004-aa8d-11e3-9046-0002a5d5c51b, service_uuid: b5f90001-aa8d-11e3-9046-0002a5d5c51b, properties: WRITE }
  Characteristic { uuid: b5f90005-aa8d-11e3-9046-0002a5d5c51b, service_uuid: b5f90001-aa8d-11e3-9046-0002a5d5c51b, properties: READ | INDICATE }
  Characteristic { uuid: b5f90006-aa8d-11e3-9046-0002a5d5c51b, service_uuid: b5f90001-aa8d-11e3-9046-0002a5d5c51b, properties: READ }
Service UUID b5f90080-aa8d-11e3-9046-0002a5d5c51b, primary: true
  Characteristic { uuid: b5f90081-aa8d-11e3-9046-0002a5d5c51b, service_uuid: b5f90080-aa8d-11e3-9046-0002a5d5c51b, properties: NOTIFY }
  Characteristic { uuid: b5f90082-aa8d-11e3-9046-0002a5d5c51b, service_uuid: b5f90080-aa8d-11e3-9046-0002a5d5c51b, properties: WRITE }
  Characteristic { uuid: b5f90083-aa8d-11e3-9046-0002a5d5c51b, service_uuid: b5f90080-aa8d-11e3-9046-0002a5d5c51b, properties: NOTIFY }
  Characteristic { uuid: b5f90084-aa8d-11e3-9046-0002a5d5c51b, service_uuid: b5f90080-aa8d-11e3-9046-0002a5d5c51b, properties: NOTIFY }
Service UUID b5f90090-aa8d-11e3-9046-0002a5d5c51b, primary: true
  Characteristic { uuid: b5f90091-aa8d-11e3-9046-0002a5d5c51b, service_uuid: b5f90090-aa8d-11e3-9046-0002a5d5c51b, properties: WRITE }
  Characteristic { uuid: b5f90092-aa8d-11e3-9046-0002a5d5c51b, service_uuid: b5f90090-aa8d-11e3-9046-0002a5d5c51b, properties: NOTIFY }

 */

pub struct GoproBluetooth {
    gopro: Peripheral,
}

impl GoproBluetooth {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new().await.unwrap();

        // get the first bluetooth adapter
        let adapter = manager
            .adapters()
            .await
            .expect("Unable to fetch adapter list.")
            .into_iter()
            .next()
            .expect("Unable to find adapters.");

        // start scanning for devices
        info!("Scanning...");
        adapter.start_scan(ScanFilter::default()).await?;

        // todo: event driven discovery
        time::sleep(Duration::from_secs(2)).await;

        let gopro = find_gopro(&adapter).await.expect("No gopro found");

        info!("Found.");

        info!("Connecting...");
        gopro.connect().await?;

        gopro.discover_services().await?;
        info!("Services discovered");

        Ok(Self { gopro })
    }

    pub async fn start_ap(&self) -> Result<(), Box<dyn Error>> {
        // find the characteristic we want
        let chars = self.gopro.characteristics();
        let wifi_ssid_char = chars
            .iter()
            .find(|c| c.uuid == Uuid::from_str("b5f90002-aa8d-11e3-9046-0002a5d5c51b").unwrap())
            .expect("Unable to find characterics");

        let wifi_pass_char = chars
            .iter()
            .find(|c| c.uuid == Uuid::from_str("b5f90003-aa8d-11e3-9046-0002a5d5c51b").unwrap())
            .expect("Unable to find characterics");

        let wifi_ap_power_char = chars
            .iter()
            .find(|c| c.uuid == Uuid::from_str("b5f90004-aa8d-11e3-9046-0002a5d5c51b").unwrap())
            .expect("Unable to find characterics");

        let wifi_api_state_char = chars
            .iter()
            .find(|c| c.uuid == Uuid::from_str("b5f90005-aa8d-11e3-9046-0002a5d5c51b").unwrap())
            .expect("Unable to find characterics");

        info!(
            "SSID: {:?}",
            String::from_utf8_lossy(&self.gopro.read(wifi_ssid_char).await?)
        );

        info!(
            "Password: {:?}",
            String::from_utf8_lossy(&self.gopro.read(wifi_pass_char).await?)
        );

        let state = self.gopro.read(wifi_api_state_char).await?;
        let state = *state.first().unwrap_or(&0);

        if state != 0x00 {
            info!("AP is already enabled");
            return Ok(());
        }

        info!("AP State: {:?}", state);

        info!("Enabling access point...");
        self.gopro
            .write(wifi_ap_power_char, &[0x01], WriteType::WithoutResponse)
            .await?;

        loop {
            let state = self.gopro.read(wifi_api_state_char).await?;
            let state = *state.first().unwrap_or(&0);

            if state != 0x00 {
                info!("AP enabled");
                return Ok(());
            }

            info!("AP State: {:?}", state);

            time::sleep(Duration::from_secs(4)).await;
        }
    }

    pub async fn shutdown_camera(&self) -> Result<(), Box<dyn Error>> {
        info!("Putting camera to sleep...");
        let chars = self.gopro.characteristics();
        let commands_char = chars
            .iter()
            .find(|c| c.uuid == Uuid::from_str("b5f90072-aa8d-11e3-9046-0002a5d5c51b").unwrap())
            .expect("Unable to find characterics");

        self.gopro
            .write(commands_char, &[0x01, 0x05], WriteType::WithoutResponse)
            .await?;

        Ok(())
    }
}
