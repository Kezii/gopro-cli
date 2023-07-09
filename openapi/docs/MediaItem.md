# MediaItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**n** | **String** | Media filename | 
**cre** | **String** | Created time (seconds since epoch) | 
**r#mod** | **String** | Last modified time (seconds since epoch) | 
**s** | **String** | Size of (group) media in bytes | 
**glrv** | Option<**String**> | Low resolution video file size | [optional]
**ls** | Option<**String**> | ID of last member of a group (for grouped media items) | [optional]
**g** | Option<**String**> | Group ID (if grouped media item) | [optional]
**b** | Option<**String**> | ID of first member of a group (for grouped media items) | [optional]
**l** | Option<**String**> | ID of last member of a group (for grouped media items) | [optional]
**t** | Option<**String**> | Group type (for grouped media items) (b -> burst, c -> continuous shot, n -> night lapse, t -> time lapse) | [optional]
**m** | Option<**Vec<String>**> | List of missing/deleted group member IDs (for grouped media items) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


