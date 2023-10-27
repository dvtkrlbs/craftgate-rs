use serde::{Deserialize, Serialize};

use crate::api_error::ErrorResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    status: Option<u64>,
    #[serde(flatten)]
    pub(crate) response: ApiResponseVariant<T>
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ApiResponseVariant<T> {
    #[serde(rename = "errors")]
    Error(ErrorResponse),
    #[serde(rename = "data")]
    Success(SuccessResponse<T>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum SuccessResponse<T> {
    Single(T),
    Paginated(PaginatedResponse<T>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: usize,
    pub size: usize,
    pub total_size: usize,
}

#[cfg(test)]
mod tests {
    

    use assert_matches::assert_matches;
    use serde::{Deserialize, Serialize};

    use crate::{
        api_error::ErrorGroup,
        response::{ApiResponse, SuccessResponse},
    };
    use crate::response::ApiResponseVariant;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TestData {
        id: usize,
        created_date: String,
    }
    #[test]
    fn deserializes_errors_with_error_group() {
        let input = r#"
        {
            "errors": {
                "errorCode": "10051",
                "errorDescription": "Insufficient card limit, insufficient balance",
                "errorGroup": "NOT_SUFFICIENT_FUNDS"
            }
        }
        "#;
        let resp: ApiResponse<()> = serde_json::from_str(input).unwrap();

        assert_matches!(resp.response, ApiResponseVariant::Error(e) => {
            assert_eq!(e.code.as_ref(), &10051);
            assert_matches!(e.group, Some(ErrorGroup::NotSufficientFunds));
        })
    }

    #[test]
    fn deserializes_errors_without_error_group() {
        let input = r#"
        {
            "errors": {
                "errorCode": "4152",
                "errorDescription": "Invalid card expiry year \"2010\" "
            }
        }
        "#;

        let resp: ApiResponse<()> = serde_json::from_str(input).unwrap();

        assert_matches!(resp.response, ApiResponseVariant::Error(e) => {
            assert_eq!(e.code.as_ref(), &4152);
            assert_eq!(e.group, None);
        })
    }

    #[test]
    fn deserializes_data() {
        let input = r#"
        {
            "data": {
                "id": 5,
                "createdDate": "2021-11-15T14:07:18"
            }
        }
        "#;

        let resp: ApiResponse<TestData> = serde_json::from_str(input).unwrap();
        assert_matches!(resp.response, ApiResponseVariant::Success(succ) => {
            assert_matches!(succ, SuccessResponse::Single(d) => {
                assert_eq!(d.id, 5);
                assert_eq!(d.created_date, "2021-11-15T14:07:18".to_owned());
            });
            }
        )
    }

    #[test]
    fn deserializes_paginated_data() {
        let input = r#"
        {
            "data": {
                "items": [
                    {
                        "id": 5,
                        "createdDate": "2021-11-15T14:07:18"
                    },
                    {
                        "id": 4,
                        "createdDate": "2021-11-15T14:07:18"
                    },
                    {
                        "id": 3,
                        "createdDate": "2021-11-15T14:07:18"
                    }
                ],
                "page": 0,
                "size": 25,
                "totalSize": 3
            }
        }
        "#;

        let resp: ApiResponse<TestData> = serde_json::from_str(input).unwrap();
        assert_matches!(resp.response, ApiResponseVariant::Success(succ) => {
            assert_matches!(succ, SuccessResponse::Paginated(d) => {
                assert_eq!(d.page, 0);
                assert_eq!(d.size, 25);
                assert_eq!(d.total_size, 3);
                assert_eq!(d.items.len(), 3);
            })
        })
    }
}
