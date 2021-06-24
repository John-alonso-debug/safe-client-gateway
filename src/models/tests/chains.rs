use crate::models::chains::{ChainInfo, GasPriceOracle, NativeCurrency, Theme};
use rocket::serde::json::json;

#[test]
fn chain_info_json() {
    let chain_info_json = json!({
        "chainId": "4",
        "chainName": "Rinkeby",
        "rpcUrl": "https://someurl.com/rpc",
        "blockExplorerUrl": "https://blockexplorer.com",
        "transactionService": "https://safe-transaction.rinkeby.staging.gnosisdev.com",
        "nativeCurrency": {
            "name": "Ether",
            "symbol": "ETH",
            "decimals": 18
        },
        "theme": {
            "textColor": "#fff",
            "backgroundColor": "#000"
        },
        "gasPriceOracle": {
            "url": "https://gaspriceoracle.com/",
            "gasParameter": "average"
        }
    });

    let expected = ChainInfo {
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        rpc_url: "https://someurl.com/rpc".to_string(),
        block_explorer_url: "https://blockexplorer.com".to_string(),
        native_currency: NativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
        },
        theme: Theme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
        gas_price_oracle: Some(GasPriceOracle {
            url: "https://gaspriceoracle.com/".to_string(),
            gas_parameter: Some("average".to_string()),
        }),
    };

    let actual = serde_json::from_str::<ChainInfo>(&chain_info_json.to_string());

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_with_null_gas_price_oracle() {
    let chain_info_json = json!({
        "chainId": "4",
        "chainName": "Rinkeby",
        "rpcUrl": "https://someurl.com/rpc",
        "blockExplorerUrl": "https://blockexplorer.com",
        "transactionService": "https://safe-transaction.rinkeby.staging.gnosisdev.com",
        "nativeCurrency": {
            "name": "Ether",
            "symbol": "ETH",
            "decimals": 18
        },
        "theme": {
            "textColor": "#fff",
            "backgroundColor": "#000"
        },
    });

    let expected = ChainInfo {
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        rpc_url: "https://someurl.com/rpc".to_string(),
        block_explorer_url: "https://blockexplorer.com".to_string(),
        native_currency: NativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
        },
        theme: Theme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
        gas_price_oracle: None,
    };

    let actual = serde_json::from_str::<ChainInfo>(&chain_info_json.to_string());

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_with_null_gas_price_oracle_parameter() {
    let chain_info_json = json!({
        "chainId": "4",
        "chainName": "Rinkeby",
        "rpcUrl": "https://someurl.com/rpc",
        "blockExplorerUrl": "https://blockexplorer.com",
        "transactionService": "https://safe-transaction.rinkeby.staging.gnosisdev.com",
        "nativeCurrency": {
            "name": "Ether",
            "symbol": "ETH",
            "decimals": 18
        },
        "theme": {
            "textColor": "#fff",
            "backgroundColor": "#000"
        },
        "gasPriceOracle": {
            "url": "https://gaspriceoracle.com/",
            "gasParameter": null
        },
    });

    let expected = ChainInfo {
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        rpc_url: "https://someurl.com/rpc".to_string(),
        block_explorer_url: "https://blockexplorer.com".to_string(),
        native_currency: NativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
        },
        theme: Theme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
        gas_price_oracle: Some(GasPriceOracle {
            url: "https://gaspriceoracle.com/".to_string(),
            gas_parameter: None,
        }),
    };

    let actual = serde_json::from_str::<ChainInfo>(&chain_info_json.to_string());

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}
