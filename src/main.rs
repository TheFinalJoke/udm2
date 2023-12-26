use lib::rpc_types::fhs_types;
fn main() {
    let fhs = fhs_types::FluidRegulator{
        fr_id: 1,
        gpio_pin: 42,
        regulator_type: 0,
    };
    println!("{:?}", fhs);
}
