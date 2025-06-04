#[derive(Debug, PartialEq, Clone)]
pub enum TransactionType {
    Debit,
    Credit,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub date: String,
    pub description: String,
    pub amount: f64,
    pub transaction_type: TransactionType,
}

pub mod database {
    use leptos::prelude::*;
    use std::collections::HashMap;

    use crate::components::{
        general::{
            table::data_table::{Column, TableCellData},
            tag::LabelTag,
        },
        schemas::props::ColorTemperature,
    };

    use super::{Transaction, TransactionType};

    // Some mock data for testing purposes
    pub fn transactions() -> Vec<Transaction> {
        vec![
            Transaction {
                id: "TXN0011".to_string(),
                date: "2025-06-04T11:00:00+03:00".to_string(),
                description: "Grocery Store".to_string(),
                amount: 20.50,
                transaction_type: TransactionType::Debit,
            },
            Transaction {
                id: "TXN0012".to_string(),
                date: "2023-10-01T00:00:00+03:00".to_string(),
                description: "Template 1".to_string(),
                amount: 2000.50,
                transaction_type: TransactionType::Credit,
            },
            Transaction {
                id: "TXN0021".to_string(),
                date: "2023-10-02T00:00:00+03:00".to_string(),
                description: "Template 2".to_string(),
                amount: 1500.00,
                transaction_type: TransactionType::Credit,
            },
            Transaction {
                id: "TXN0022".to_string(),
                date: "2023-10-02T00:00:00+03:00".to_string(),
                description: "Salary".to_string(),
                amount: 1000.00,
                transaction_type: TransactionType::Debit,
            },
            Transaction {
                id: "TXN0031".to_string(),
                date: "2023-10-03T00:00:00+03:00".to_string(),
                description: "Electric Bill".to_string(),
                amount: 75.00,
                transaction_type: TransactionType::Debit,
            },
            Transaction {
                id: "TXN0032".to_string(),
                date: "2023-10-03T00:00:00+03:00".to_string(),
                description: "Template 1".to_string(),
                amount: 2000.00,
                transaction_type: TransactionType::Credit,
            },
            Transaction {
                id: "TXN0041".to_string(),
                date: "2023-10-04T00:00:00+03:00".to_string(),
                description: "Dining Out".to_string(),
                amount: 500.25,
                transaction_type: TransactionType::Debit,
            },
            Transaction {
                id: "TXN0042".to_string(),
                date: "2023-10-04T00:00:00+03:00".to_string(),
                description: "Tax Refund".to_string(),
                amount: 1005.25,
                transaction_type: TransactionType::Credit,
            },
            Transaction {
                id: "TXN005".to_string(),
                date: "2023-10-05T00:00:00+03:00".to_string(),
                description: "Netflix Subscription".to_string(),
                amount: 12.99,
                transaction_type: TransactionType::Debit,
            },
            // New transactions start here
            Transaction {
                id: "TXN0061".to_string(),
                date: "2023-10-05T00:00:00+03:00".to_string(),
                description: "Gas Station".to_string(),
                amount: 45.30,
                transaction_type: TransactionType::Debit,
            },
            Transaction {
                id: "TXN0062".to_string(),
                date: "2023-10-05T00:00:00+03:00".to_string(),
                description: "Freelance Payment".to_string(),
                amount: 800.00,
                transaction_type: TransactionType::Credit,
            },
            Transaction {
                id: "TXN0071".to_string(),
                date: "2023-10-06T00:00:00+03:00".to_string(),
                description: "Internet Bill".to_string(),
                amount: 60.00,
                transaction_type: TransactionType::Debit,
            },
            Transaction {
                id: "TXN0072".to_string(),
                date: "2023-10-06T00:00:00+03:00".to_string(),
                description: "Investment Dividend".to_string(),
                amount: 250.75,
                transaction_type: TransactionType::Credit,
            },
            Transaction {
                id: "TXN0081".to_string(),
                date: "2023-10-07T00:00:00+03:00".to_string(),
                description: "Coffee Shop".to_string(),
                amount: 8.99,
                transaction_type: TransactionType::Debit,
            },
            Transaction {
                id: "TXN0082".to_string(),
                date: "2023-10-07T00:00:00+03:00".to_string(),
                description: "Template 2".to_string(),
                amount: 1750.00,
                transaction_type: TransactionType::Credit,
            },
            Transaction {
                id: "TXN0091".to_string(),
                date: "2023-10-08T00:00:00+03:00".to_string(),
                description: "Gym Membership".to_string(),
                amount: 35.00,
                transaction_type: TransactionType::Debit,
            },
            Transaction {
                id: "TXN0092".to_string(),
                date: "2023-10-08T00:00:00+03:00".to_string(),
                description: "Rental Income".to_string(),
                amount: 1200.00,
                transaction_type: TransactionType::Credit,
            },
            Transaction {
                id: "TXN0101".to_string(),
                date: "2023-10-09T00:00:00+03:00".to_string(),
                description: "Online Shopping".to_string(),
                amount: 120.45,
                transaction_type: TransactionType::Debit,
            },
            Transaction {
                id: "TXN0102".to_string(),
                date: "2023-10-09T00:00:00+03:00".to_string(),
                description: "Cashback Reward".to_string(),
                amount: 50.00,
                transaction_type: TransactionType::Credit,
            },
            Transaction {
                id: "TXN0111".to_string(),
                date: "2023-10-10T00:00:00+03:00".to_string(),
                description: "Water Bill".to_string(),
                amount: 25.75,
                transaction_type: TransactionType::Debit,
            },
            Transaction {
                id: "TXN0112".to_string(),
                date: "2023-10-10T00:00:00+03:00".to_string(),
                description: "Consulting Fee".to_string(),
                amount: 950.00,
                transaction_type: TransactionType::Credit,
            },
            Transaction {
                id: "TXN0121".to_string(),
                date: "2023-10-10T00:00:00+03:00".to_string(),
                description: "Movie Tickets".to_string(),
                amount: 30.00,
                transaction_type: TransactionType::Debit,
            },
            Transaction {
                id: "TXN0122".to_string(),
                date: "2023-10-10T00:00:00+03:00".to_string(),
                description: "Template 1".to_string(),
                amount: 2100.00,
                transaction_type: TransactionType::Credit,
            },
            Transaction {
                id: "TXN0131".to_string(),
                date: "2023-10-10T00:00:00+03:00".to_string(),
                description: "Phone Bill".to_string(),
                amount: 55.20,
                transaction_type: TransactionType::Debit,
            },
        ]
    }

    pub fn get_transactions() -> (Vec<Column>, Vec<HashMap<String, TableCellData>>) {
        let columns = vec![
            Column::new("Transaction ID", false),
            Column::new("Date", true),
            Column::new("Duration", true),
            Column::new("Description", true),
            Column::new("Transaction Type", false),
            Column::new("Amount", true),
        ];

        let transactions = transactions()
            .iter()
            .map(|transaction| {
                let mut hash_map_data = HashMap::new();

                hash_map_data.insert(
                    "id".to_string(),
                    TableCellData::String(transaction.id.clone()),
                );

                hash_map_data.insert(
                    "Transaction ID".to_string(),
                    TableCellData::String(transaction.id.clone()),
                );
                hash_map_data.insert(
                    "Date".to_string(),
                    TableCellData::DateTime(transaction.date.clone()),
                );
                hash_map_data.insert(
                    "Duration".to_string(),
                    TableCellData::Duration(transaction.date.clone()),
                );
                hash_map_data.insert(
                    "Description".to_string(),
                    TableCellData::String(transaction.description.clone()),
                );
                hash_map_data.insert(
                    "Transaction Type".to_string(),
                    TableCellData::Html(
                        match transaction.transaction_type {
                            TransactionType::Credit => (|| view!{ <LabelTag label="Credit".to_string() color=ColorTemperature::Info  /> }).into(),
                            TransactionType::Debit => (|| view!{ <LabelTag label="Debit".to_string() color=ColorTemperature::Warning  /> }).into(),
                        }
                    ),
                );
                hash_map_data.insert(
                    "Amount".to_string(),
                    TableCellData::Float64(transaction.amount.clone()),
                );

                hash_map_data
            })
            .collect();

        (columns, transactions)
    }
}
