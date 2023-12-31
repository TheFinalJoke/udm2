// use crate::db::{sqlite, SqlTransactions};
tonic::include_proto!("fhs_types");
// const TABLE_NAME: &str = "FluidRegulation";

// impl SqlTransactions for FluidRegulator {
//     fn add(&self) -> String {
//         let values = format!(
//             "({id}, {gpio_pin}, {reg_type})",
//             id = self.fr_id,
//             gpio_pin = self.gpio_pin,
//             reg_type = self.regulator_type
//         );
//         sqlite::insert_transaction_sql_generator(
//             TABLE_NAME,
//             "(fr_id, gpio_pin, regulator_type)",
//             &values,
//         )
//     }

//     fn modify(&self) -> String {
//         let set_values = format!(
//             "gpio_pin = '{}', regulator_type = '{}'",
//             &self.gpio_pin, &self.regulator_type
//         );
//         sqlite::update_transaction_sql_generator(
//             TABLE_NAME,
//             set_values.as_str(),
//             format!("fr_id={}", self.fr_id).as_str(),
//         )
//     }

//     fn drop(&self) -> String {
//         sqlite::delete_transaction_sql_generator(
//             TABLE_NAME,
//             format!("fr_uid={}", self.fr_id).as_str(),
//         )
//     }

//     fn get(&self) -> String {
//         todo!()
//     }
// }
