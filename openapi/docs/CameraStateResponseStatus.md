# CameraStateResponseStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**param_1** | **i32** | Is the system’s internal battery present? | 
**param_2** | **i32** | Rough approximation of internal battery level in bars. | 
**param_6** | **i32** | Is the system currently overheating? | 
**param_8** | **i32** | Is the camera busy? | 
**param_9** | **i32** | Is Quick Capture feature enabled? | 
**param_10** | **i32** | Is the system encoding right now? | 
**param_11** | **i32** | Is LCD lock active? | 
**param_13** | **i32** | When encoding video, this is the duration (seconds) of the video so far; 0 otherwise. | 
**param_17** | **i32** | Are Wireless Connections enabled? | 
**param_19** | **i32** | The pairing state of the camera. | 
**param_20** | **i32** | The last type of pairing that the camera was engaged in. | 
**param_21** | **i32** | Time (milliseconds) since boot of last successful pairing complete action. | 
**param_22** | **i32** | State of current scan for WiFi Access Points. Appears to only change for CAH-related scans. | 
**param_23** | **i32** | The time, in milliseconds since boot that the WiFi Access Point scan completed. | 
**param_24** | **i32** | WiFi AP provisioning state. | 
**param_26** | **i32** | Wireless remote control version. | 
**param_27** | **i32** | Is a wireless remote control connected? | 
**param_28** | **i32** | Wireless Pairing State. | 
**param_29** | **String** | Provisioned WIFI AP SSID. On BLE connection, value is big-endian byte-encoded int. | 
**param_30** | **String** | Camera’s WIFI SSID. On BLE connection, value is big-endian byte-encoded int. | 
**param_31** | **i32** | The number of wireless devices connected to the camera. | 
**param_32** | **i32** | Is Preview Stream enabled? | 
**param_33** | **i32** | Primary Storage Status. | 
**param_34** | **i32** | How many photos can be taken before sdcard is full | 
**param_35** | **i32** | How many minutes of video can be captured with current settings before sdcard is full | 
**param_36** | **i32** | How many group photos can be taken with current settings before sdcard is full | 
**param_37** | **i32** | Total number of group videos on sdcard | 
**param_38** | **i32** | Total MB of space on the sdcard. | 
**param_39** | **i32** | Number of photos on sdcard. | 
**param_40** | Option<**i32**> | Number of videos on sdcard. | [optional]
**param_41** | **i32** | OTA status. | 
**param_70** | **i32** | Internal battery level (percent) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


