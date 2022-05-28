# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_install_post**](DefaultApi.md#app_install_post) | **POST** /app/install | Install an app from the FLECS marketplace
[**app_list_get**](DefaultApi.md#app_list_get) | **GET** /app/list | List installed apps and their instances
[**app_uninstall_post**](DefaultApi.md#app_uninstall_post) | **POST** /app/uninstall | Uninstall an app
[**instance_create_post**](DefaultApi.md#instance_create_post) | **POST** /instance/create | Create new instance of an installed app
[**instance_delete_post**](DefaultApi.md#instance_delete_post) | **POST** /instance/delete | Delete an app instance
[**instance_details_post**](DefaultApi.md#instance_details_post) | **POST** /instance/details | Obtain details of an app instance
[**instance_start_post**](DefaultApi.md#instance_start_post) | **POST** /instance/start | Start an app instance
[**instance_stop_post**](DefaultApi.md#instance_stop_post) | **POST** /instance/stop | Stop an app instance
[**system_ping_get**](DefaultApi.md#system_ping_get) | **GET** /system/ping | Check daemon availability and connectivity



## app_install_post

> crate::models::AppInstallPost200Response app_install_post(app_install_post_request)
Install an app from the FLECS marketplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_install_post_request** | [**AppInstallPostRequest**](AppInstallPostRequest.md) |  | [required] |

### Return type

[**crate::models::AppInstallPost200Response**](_app_install_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_list_get

> crate::models::AppListGet200Response app_list_get()
List installed apps and their instances

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AppListGet200Response**](_app_list_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_uninstall_post

> crate::models::AppInstallPost200Response app_uninstall_post(app_uninstall_post_request)
Uninstall an app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_uninstall_post_request** | [**AppUninstallPostRequest**](AppUninstallPostRequest.md) |  | [required] |

### Return type

[**crate::models::AppInstallPost200Response**](_app_install_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_create_post

> crate::models::InstanceCreatePost200Response instance_create_post(instance_create_post_request)
Create new instance of an installed app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_create_post_request** | [**InstanceCreatePostRequest**](InstanceCreatePostRequest.md) |  | [required] |

### Return type

[**crate::models::InstanceCreatePost200Response**](_instance_create_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_delete_post

> crate::models::InstanceDeletePost200Response instance_delete_post(instance_delete_post_request)
Delete an app instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_delete_post_request** | [**InstanceDeletePostRequest**](InstanceDeletePostRequest.md) |  | [required] |

### Return type

[**crate::models::InstanceDeletePost200Response**](_instance_delete_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_details_post

> crate::models::InstanceDetailsPost200Response instance_details_post(instance_id)
Obtain details of an app instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | [**InstanceId**](InstanceId.md) |  | [required] |

### Return type

[**crate::models::InstanceDetailsPost200Response**](_instance_details_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_start_post

> crate::models::InstanceDeletePost200Response instance_start_post(instance_delete_post_request)
Start an app instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_delete_post_request** | [**InstanceDeletePostRequest**](InstanceDeletePostRequest.md) |  | [required] |

### Return type

[**crate::models::InstanceDeletePost200Response**](_instance_delete_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_stop_post

> crate::models::InstanceDeletePost200Response instance_stop_post(instance_delete_post_request)
Stop an app instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_delete_post_request** | [**InstanceDeletePostRequest**](InstanceDeletePostRequest.md) |  | [required] |

### Return type

[**crate::models::InstanceDeletePost200Response**](_instance_delete_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_ping_get

> crate::models::SystemPingGet200Response system_ping_get()
Check daemon availability and connectivity

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SystemPingGet200Response**](_system_ping_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

